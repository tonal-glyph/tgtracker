//@ - ruckus - A ChucK creative coding toolkit for Rust
//@     - [x] ruck - passes system command flag and all other arguments to `chuck` binary
//@         - [x] Call Rust from ChucK
//@         - [x] Call ChucK from Rust
//@     - [x] Test faust2ck, it generates chugins from FAUST code
//@     - [x] Test Faust ugen, it processes inline FAUST code
//@     - [ ] MIDI
//@         - [ ] midir - replaces RtMidi
//@             - [ ] sysex
//@     - [ ] chuck-sys
//@         - [ ] alsa crate
//@         - [ ] cpal/rsoundio replaces RtAudio
//@         - [ ] jack crate
//@         - [ ] pulse-simple crate
//@         - [ ] Write C wrapper for libchuck
//@         - [ ] Instantiate ChucK VMs using libchuck
//@         - [ ] Connect libchuck VMs to audio IO
//@     - [ ] OSC
//@         - [ ] osc_address - type-safe abstraction over serde_osc
//@         - [ ] rosc - replaces liblo and CNMAT OSC-Kit
//@         - [ ] serde_osc - serialize OSC data
//@         - [ ] serde_bytes - for wrapping OSC blobs
//@     - [ ] HID
//@         - [ ] sdl2 crate
//@     - [ ] serial
//@         - [ ] serial crate
//@     - [ ] write C wrapper for STK newest version
//@     - [ ] sndfile-sys - libsndfile
