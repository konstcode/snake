use crossterm::{
    cursor::{Hide, Show},
    event,
    event::{Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use snake::frame::{new_frame, Drawable, Frame};
use snake::render;
use snake::snake::Snake;
use snake::{audio::Audio, snake::Direction};
use std::{
    error::Error,
    io,
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

static AUDIO_DIR: &str = "audio";

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
    let mut snake = Snake::new();

    'gameloop: loop {
        // Per-frame init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();

        // Input handlers for the game
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("win");
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

        snake.update(delta);
        if snake.is_dead() {
            audio.play("win");
            break 'gameloop;
        }
        snake.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
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
