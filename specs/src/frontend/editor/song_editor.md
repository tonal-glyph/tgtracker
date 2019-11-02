# song editor

The 'song editor' is a tracker-style interface for editing patterns of note data.

- [ ] Use glyph_brush and rusttype for font rendering
- [ ] Pattern scroll (as opposed to piano roll) i.e. chiptune/demoscene/tracker style
- [ ] Zoom in/out
- [ ] Standard pattern length is 64 notes, controllable by up/down buttons and a text field
- [ ] Add/remove channels (tracks) at will, use flexbox to fit many channels on 1 screen
- [ ] Have a side-by-side multi-track view and a single track zen-coding view
- [ ] Notes can trigger multiple events in tgtracker
    - [ ] Playing a sound file (recording, "sample") at a notated key and octave, C-4
    - [ ] Playing a generator plugin at the given key and octave
    - [ ] Load a ChucK file
    - [ ] Load a FAUST file
    - [ ] Send MIDI data
    - [ ] Send/receive OSC data
    - [ ] Mute or fade out the channel

```rust
#![allow(unused_imports)]
use crate::core::pattern::effects::{Effect, EffectArgs};
use crate::core::pattern::note::{BaseNote, Coordinates, Key, Note};
use crate::core::pattern::pattern::{Gain, Pattern, Points};
use crate::core::events::triggers::{Destination, Source, TGTEvent, TGTEventData, TGTEventType, TransportEvent};
use crate::core::chuck::{ChucK, ChucKOps};
use crate::core::backends::busses::Busses;
use crate::core::settings::{CurrentTheme, Interpolation, KeybindMode, Octave, SampleBitDepth, SampleRate, Tuning};
```