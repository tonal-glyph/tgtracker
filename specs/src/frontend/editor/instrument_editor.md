# instrument mod

An instrument in tgtracker is something that makes noise, be it an audio clip, video clip, VST/LV2 plugin, ChucK program, FAUST program, or something else.

The instrument editor allows the user to control instrument-specific settings like paths, panning, gain, envelopes, MIDI control numbers...

- [ ] File path to sound file/video/plugin/code
- [ ] Panning
- [ ] Gain
- [ ] Envelopes
- [ ] MIDI channel

```rust
#![allow(dead_code)]
use std::path::PathBuf;
pub struct Instrument {
    file: Option<PathBuf>,
}
```