//@ - [ ] Settings specific to a channel
//@     - [ ] Volume/gain
//@     - [ ] Panning
//@     - [ ] Generator plugins (VSTi, AU, LV2, etc.)
//@     - [ ] Transformer plugins (VST, AU, Ladspa, LV2, FAUST code, etc.)
//@     - [ ] MrsWatson for offline processing
//@     - [ ] Current bus (default Master)
//@     - [ ] Routing matrix
//@     - [ ] Automation envelopes
//@     - [ ] MIDI CC messages

pub struct Channel {
    pub gain: f64,
    pub pan: f64,
    // pub generators: Vec<Generator>,
    // pub transformers: Vec<Transformer>,
    pub offline: bool,
    pub bus: u32,
}
