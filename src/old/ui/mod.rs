#[macro_use] use nannou::ui::conrod::*;
use support;
pub fn main() {
    feature::main();
}
mod feature {
    use nannou::find_folder;
    extern crate image;
    use conrod;
    use conrod::backend::glium::glium::{self, Surface};
    use support;
    use std;
    const WIN_W: u32 = support::WIN_W;
    const WIN_H: u32 = support::WIN_H;
    pub fn main() {
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("Conrod with glium!")
            .with_dimensions((WIN_W, WIN_H).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(4);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let mut ui = conrod::UiBuilder::new([WIN_W as f64, WIN_H as f64]).theme(support::theme()).build();
        let ids = support::Ids::new(ui.widget_id_generator());
        let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
        let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(font_path).unwrap();
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let mut event_loop = support::EventLoop::new();
        'main: loop {
            for event in event_loop.next(&mut events_loop) {
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }
                match event {
                    glium::glutin::Event::WindowEvent { event, .. } => match event {
                        glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'main,
                        _ => (),
                    },
                    _ => (),
                }
            }
            use nannou::app;
            support::gui(&mut ui.set_widgets(), &ids, &mut app);
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
}
