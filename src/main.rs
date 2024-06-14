use crossterm::{
    cursor::{Hide, Show},
    event,
    event::{Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rand::{thread_rng, Rng};
use snake::{
    apple::AppleDispencer,
    frame::{new_frame, Drawable, Frame},
};
use snake::{audio::Audio, snake::Direction};
use snake::{menu::Menu, snake::Snake};
use snake::{render, topbar::TopBar};
use std::{
    error::Error,
    io,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

static AUDIO_DIR: &str = "audio";
const MAX_APPLES: u8 = 3;
const SPEED: u64 = 300;

fn render_screen(render_rx: Receiver<Frame>) {
    let mut last_frame = new_frame();
    let mut stdout = io::stdout();
    render::render(&mut stdout, &last_frame, &last_frame, true);
    while let Ok(curr_frame) = render_rx.recv() {
        render::render(&mut stdout, &last_frame, &curr_frame, false);
        last_frame = curr_frame;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.init(AUDIO_DIR);

    // Terminal
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in a separate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        render_screen(render_rx);
    });

    // Gameloop
    let mut instant = Instant::now();
    let mut menu = Menu::new(SPEED, MAX_APPLES);

    audio.play("enter");
    'menuloop: loop {
        // Per-frame init
        let mut curr_frame = new_frame();

        // Input hadleres for menu
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Enter => {
                        menu.active = false;
                        render::render(&mut stdout, &curr_frame, &curr_frame, true);
                    }
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'menuloop;
                    }
                    KeyCode::Up | KeyCode::Down => menu.switch_current_option(),
                    KeyCode::Left => menu.decrease_current_option(),
                    KeyCode::Right => menu.insrease_current_option(),
                    _ => {}
                }
            }
        }
        menu.draw(&mut curr_frame);
        if !menu.active {
            let mut snake = Snake::new(menu.speed());
            let mut apple_dispencer = AppleDispencer::new(menu.apples());
            let mut topbar = TopBar::new();

            'gameloop: loop {
                let delta = instant.elapsed();
                instant = Instant::now();
                curr_frame = new_frame();

                // Input handlers for the game
                while event::poll(Duration::default())? {
                    if let Event::Key(key_event) = event::read()? {
                        match key_event.code {
                            KeyCode::Esc | KeyCode::Char('q') => {
                                audio.play("lose_sound");
                                menu.active = true;
                                render::render(&mut stdout, &curr_frame, &curr_frame, true);
                                break 'gameloop;
                            }
                            KeyCode::Left => snake.turn_if_possible(Direction::Left),
                            KeyCode::Right => snake.turn_if_possible(Direction::Right),
                            KeyCode::Up => snake.turn_if_possible(Direction::Up),
                            KeyCode::Down => snake.turn_if_possible(Direction::Down),
                            _ => {}
                        }
                    }
                }

                snake.update(delta, || audio.play("move"));
                apple_dispencer.update(delta);
                snake.check_if_ate_apple(&mut apple_dispencer, || {
                    topbar.scores();
                    let mut rng = thread_rng();
                    if rng.gen() {
                        audio.play("hrum");
                    } else {
                        audio.play("niam");
                    }
                });
                if snake.is_dead() {
                    audio.play("lose_sound");
                    menu.get_game_results(topbar.get_scores(), topbar.get_time());
                    menu.active = true;
                    continue 'menuloop;
                }

                let drawables: Vec<&dyn Drawable> = vec![&snake, &apple_dispencer, &topbar];
                for drawable in drawables {
                    drawable.draw(&mut curr_frame);
                }

                let _ = render_tx.send(curr_frame);
                thread::sleep(Duration::from_millis(1));
                continue;
            }
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(10));
        continue;
    }

    // Cleanup
    drop(render_tx);
    render_handle.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
