extern crate actix;
extern crate actix_lua;

#[macro_use] extern crate conrod;
extern crate conrod_core;
#[macro_use] extern crate conrod_derive;
extern crate conrod_example_shared;
extern crate conrod_gfx;
#[macro_use] extern crate conrod_winit;

extern crate dsp;
extern crate envelope;
extern crate find_folder;

extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glutin;

extern crate glium;
extern crate glutin;
extern crate glyph_brush;
extern crate hound;
extern crate image;
extern crate keybind;
extern crate libloading;
extern crate midir;
extern crate pitch;
extern crate rand;
extern crate rb;
extern crate rosc;
extern crate rsoundio;
extern crate rustfft;
extern crate sample;
extern crate samplerate;
extern crate sdl2;

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_bytes;
extern crate serde_osc;

extern crate ttf_noto_sans;
extern crate winit;
extern crate yansi;

pub mod backend;
pub mod frontend;
mod tests;

use yansi::Paint;
// [cfg(win)]
// Paint::enable_windows_ascii();
// if cfg!(windows) && !Paint::enable_windows_ascii() {
//     Paint::disable();
// }

/// Boot main thread
fn main() {
    println!("01 initialize {}...", Paint::yellow("audio"));
    backend::audio::init::init();
    println!("02 initialize {}...", Paint::green("midi"));
    backend::audio::midi::init();
    println!("03 initialize {}...", Paint::blue("osc"));
    backend::audio::osc::init();
    println!("04 initialize {}...", Paint::cyan("ui"));
    // backend::keybindings::tgtracker_section_switching();
    frontend::conrod_ui::run();
}
