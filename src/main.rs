#![allow(dead_code)]
#![forbid(unsafe_code)]
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

use std::collections::HashMap;
use dsp::daggy::*;
use nannou::prelude::*;     // nannou prelude
use nannou::ui::prelude::*; // ui prelude
use nannou::App;
use nannou::Frame;
use conrod::backend::glium::glium::{self, Surface};
use conrod::widget::graph::{node, EdgeEvent, Event, Node, NodeEvent, NodeSocket};
use conrod::{widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
mod support;
pub mod dsp_graph;
fn main() {
    use dsp_graph;
    dsp_graph::graph();
}
