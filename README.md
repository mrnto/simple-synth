# SimpleSynth Plugin
Software synthesizer plugin written in Rust using [NIH-plug](https://github.com/robbert-vdh/nih-plug).

## Features
- Oscillator with basic waveforms
- Envelope generator (ADSR)
- Filters (low-pass, high-pass, band-pass)
- Polyphony
- ~~Two oscillators per voice~~ *TODO*
- ~~Low-frequency oscillator (LFO)~~ *TODO*
- ~~MIDI input support~~ *TODO*
- ~~GUI~~ *TODO*
- VST3 and CLAP plugin formats

## Build
Building the plugin may require certain development libraries to be installed (e.g. `libx11-dev` and `libxcursor-dev` on Linux, which are needed for the iced GUI).

To build the plugin, run:
```bash
cargo xtask bundle simple-synth --release
```

The plugin bundles will be available in the `target/bundled/` directory.
Copy or create symbolic links from `target/bundled/` to DAW’s plugin directory to load the plugin.

## License
This project is licensed under the terms of the [MIT license](https://github.com/mrnto/simple-synth/blob/main/LICENSE).
