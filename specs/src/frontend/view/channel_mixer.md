# channel mixer

Traditional mixing board style interface for managing all of a "song's" channels. VU meters and knobs would be ideal.

- [ ] Track labels
- [ ] Mute button
- [ ] Solo button
- [ ] VU meters
- [ ] Knobs/sliders
    - [ ] Gain
    - [ ] Filter
    - [ ] Panning
- [ ] Routing busses for mixdowns and stuff

```rust
#![allow(unused_variables, dead_code)]
fn set_channel_label(channel: u32, label: &str) {}
fn clear_channel_label(channel: u32) {
    // set_channel_label(channel, channel.to_str());
}
fn route_channel(channel: u32) {}
fn mute_channel(channel: u32) -> bool {
    return  true;
}
fn solo_channel(channel: u32) -> bool {
    return  true;
}
```