#![cfg(test)]
use crate::backend;
#[test]
fn test_rsoundio() { backend::audio::init::init(); }
#[test]
fn test_midi() { backend::audio::midi::init(); }
#[test]
fn test_osc() { backend::audio::osc::init(); }