#![allow(dead_code)]
#![allow(unsafe_code)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#[macro_use()]
extern crate conrod;
#[macro_use()]
extern crate conrod_derive;
#[macro_use()]
extern crate dsp;
extern crate rusttype;
extern crate nannou;
extern crate petgraph;
extern crate rand;

pub mod dsp_graph;
fn main() {
    use dsp_graph;
    dsp_graph::graph();
}
