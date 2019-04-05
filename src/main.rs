#![allow(
    dead_code,
    missing_docs,
    unsafe_code,
    unused_attributes,
    unused_imports,
    unused_variables,
)]
extern crate dsp;
extern crate glyph_brush;
extern crate lyon;
extern crate nannou;
extern crate sdl2;
extern crate stretch;
use nannou as nn;
use nn::daggy;
use nn::glium;
use nn::audio::cpal;
use nn::audio::sample;
use nn::ease::pennereq;
use nn::image;
use nn::osc::rosc;
use nn::rand;
use nn::ui::conrod;
pub mod audio;
pub mod dsp_graph;
pub mod osc;
pub mod support;
pub mod ui;
pub fn main() {
    use ui;
    ui::main();
}

/*
nannou root: reexports daggy, find_folder, glium
nannou::audio reexports cpal, sample
nannou::ease reexports pennereq
nannou::image reexports image
nannou::math reexports approx, cgmath
nannou::noise rexports noise
nannou::osc reexports rosc
nannou::rand reexports rand
nannou::ui reexports conrod
*/