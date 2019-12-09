- [ ] Note data
    - [ ] sound file note playback
    - [ ] instrument/generator note playback
    - [x] initialize ChucK script
    - [ ] initialize FAUST code
    - [ ] send MIDI data
- [ ] Transport events
    - [ ] Play song
    - [ ] Play pattern in a loop
    - [ ] Stop/pause
- [ ] Keybindings

```rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TGTEventType {
    Automation,
    Midi,
    Osc,
    Phrase,
    Script,
    Synth,
}
impl TGTEventType {
    pub const VARIANTS: [TGTEventType; 6] = [
        TGTEventType::Automation,
        TGTEventType::Midi,
        TGTEventType::Osc,
        TGTEventType::Phrase,
        TGTEventType::Script,
        TGTEventType::Synth,
    ];
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransportEvent {
    Backwards,
    Forwards,
    Loop,
    Pause,
    Play,
    Record,
    Stop,
}
impl TransportEvent {
    pub const VARIANTS: [TransportEvent; 7] = [
        TransportEvent::Backwards,
        TransportEvent::Forwards,
        TransportEvent::Loop,
        TransportEvent::Pause,
        TransportEvent::Play,
        TransportEvent::Record,
        TransportEvent::Stop,
    ];
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Destination {
    pub id: u32,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Source {
    pub id: u32,
}
#[derive(Clone, Debug)]
pub struct TGTEventData {
    dat: Vec<f64>,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct TGTEvent {
    pub dest: *const Destination,
    pub src: *const Source,
    pub dat: *const TGTEventData,
    pub etype: TGTEventType,
}
```