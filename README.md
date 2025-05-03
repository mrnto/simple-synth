# Simple synthesizer
Software synthesizer written in Rust using [CPAL](https://github.com/RustAudio/cpal) for cross-platform audio output and [Slint](https://github.com/slint-ui/slint) for a cross-platform graphical interface.

## Features
- Oscillator with basic waveforms
- Envelope generator (ADSR)
- Polyphony
- ~~Two oscillators per voice~~ *TODO*
- ~~Low-frequency oscillator (LFO)~~ *TODO*
- ~~Filters (low-pass, high-pass)~~ *TODO*
- ~~MIDI input support~~ *TODO*
- GUI

## Build and Run
### Prerequisites
On Linux, install the ALSA development files.
- For Debian/Ubuntu:
```bash
sudo apt install libasound2-dev
```
- For Fedora:
```bash
sudo dnf install alsa-lib-devel
```
On Windows **with ASIO**, follow the [instructions for using ASIO with CPAL](https://github.com/RustAudio/cpal/blob/master/README.md#asio-on-windows).

### Running
```bash
cargo run --release
```

## License
This project is licensed under the terms of the [MIT license](https://github.com/mrnto/simple-synthesizer/blob/main/LICENSE).
