#![allow(dead_code)]
#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![allow(unused_imports)]
// extern crate abscissa;
extern crate dsp;
extern crate rand;
extern crate nannou as nn;
extern crate petgraph;
#[macro_use] extern crate conrod;
extern crate rutie; // Now we can use Ruby as a scripting language!!!
mod support;
use dsp::daggy::*;
use nn::prelude::*; // nannou prelude
use nn::ui::prelude::*; // ui prelude
use nn::App;
use nn::Frame;
use rutie::{Object, RString, VM};
fn main() {
    nannou::app(model, event, view).run();
}
struct Model {}
fn model(app: &App) -> Model {
    let _window = app.new_window().with_dimensions(720, 720).build().unwrap();
    Model {}
}
fn event(_app: &App, model: Model, event: Event) -> Model {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            Moved(_pos) => {}
            KeyPressed(_key) => {}
            KeyReleased(_key) => {}
            MouseMoved(_pos) => {}
            MouseDragged(_pos, _button) => {}
            MousePressed(_button) => {}
            MouseReleased(_button) => {}
            MouseEntered => {}
            MouseExited => {}
            Resized(_size) => {}
            _other => (),
        },
        Event::Update(_dt) => {}
        _ => (),
    }
    model
}
fn view(app: &App, _model: &Model, frame: Frame) -> Frame {
    let draw = app.draw();
    draw.background().color(LIGHT_PURPLE);
    draw.to_frame(app, &frame).unwrap();
    frame
}
