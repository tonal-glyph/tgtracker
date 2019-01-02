#![allow(dead_code)]
#![allow(unsafe_code)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#[macro_use()] extern crate conrod;
#[macro_use()] extern crate conrod_derive;
#[macro_use()] extern crate dsp;
extern crate find_folder;
extern crate rusttype;
#[macro_use()] extern crate nannou;
extern crate petgraph;
extern crate rand;
pub mod dsp_graph;
pub mod editor;
pub mod fileops;
pub mod instrument;
pub mod pattern;
pub mod sample;
pub mod support;
fn main() {
    use dsp_graph;
    use editor;
    use fileops;
    use instrument;
    use pattern;
    use sample;
    dsp_graph::graph();
    editor::main();
    // fileops::main();
    instrument::main();
    pattern::main();
    sample::main();
}
