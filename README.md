Classic snake game is terminal based, open source, inspired by [invaders](https://github.com/CleanCut/invaders), to get some Rust practice and 
some basic game dev experience.

### Dependencies on Linux

Audio should work out-of-the-box on macOS, Windows, and iOS.  For Linux, the
downstream package for actually _playing_ sound ([CPAL]) requires
the *Alsa* development libraries to be installed.

**CentOS**

```bash
sudo yum install -y alsa-lib-devel
```

**Debian/Ubuntu**

```bash
sudo apt install libasound2-dev pkg-config
```
**Arch Linux**

```bash
sudo pacman -S alsa-lib pkgconf libx11
```
You will also need `pipewire-alsa` or `pulseaudio-alsa` depending on the sound server you are using.
