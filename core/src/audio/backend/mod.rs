#![cfg(test)]
/// MIDI input/output with `midir`
pub mod midi;

/// libsoundio bindings
pub mod sio;
/// RTAudio bindings
// pub mod rta;
/// PortAudio bindings
// pub mod porta;

mod test {
    #[test]
    fn test_sio() {
        use super::sio;
        sio::init();
    }
    #[test]
    fn test_rta() {}
    #[test]
    fn test_porta() {}
}
