#![allow(dead_code)]
#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![allow(unused_imports)]

// extern crate abscissa;
extern crate dsp;
extern crate nannou;

use dsp::daggy::*;
use nannou::prelude::*;
use nannou::ui::prelude::widget::graph;
use nannou::ui::prelude::*;
use std::env;

fn main() {
    nannou::run(model, event, view);
}

struct Model {
    window: WindowId,
}

fn model(app: &App) -> Model {
    // Set the loop mode to wait for events, an energy-efficient option for
    // pure-GUI apps.
    app.set_loop_mode(LoopMode::wait(3));

    // Create the UI.
    // let mut ui = app.new_ui().build().unwrap();

    let window = app
        .new_window()
        .with_title("tgtracker 0.1.0-alpha")
        .build()
        .unwrap();

    Model { window }
}

fn event(_app: &App, model: Model, event: Event) -> Model {
    match event {
        Event::WindowEvent {
            simple: Some(event),
            ..
        } => match event {
            Moved(_pos) => {
                // println!("{:?}", event);
            }

            KeyPressed(_key) => {
                // println!("{:?}", event);
            }

            KeyReleased(_key) => {
                // println!("{:?}", event);
            }

            MouseMoved(_pos) => {
                // println!("{:?}", event);
            }

            MouseDragged(_pos, _button) => {
                // println!("{:?}", event);
            }

            MousePressed(_button) => {
                // println!("{:?}", event);
            }

            MouseReleased(_button) => {
                // println!("{:?}", event);
            }

            MouseEntered => {
                // println!("{:?}", event);
            }

            MouseExited => {
                // println!("{:?}", event);
            }

            Resized(_size) => {
                // println!("{:?}", event);
            }

            _other => (),
        },

        Event::Update(_dt) => {}

        _ => (),
    }
    model
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) -> Frame {
    frame.window(model.window).unwrap().clear(DARK_CHARCOAL);
    // Begin drawing
    // let draw = app.draw();

    // draw.background().rgb(0.02, 0.02, 0.02);

    // // Write the result of our drawing to the window's OpenGL frame.
    // draw.to_frame(app, &frame).unwrap();

    // // Draw the state of the `Ui` to the frame.
    // model.ui.draw_to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
