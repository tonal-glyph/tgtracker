extern crate conrod_core;
extern crate conrod_gfx;
extern crate conrod_winit;
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate find_folder;
extern crate image;
extern crate rand;
extern crate winit;
pub const WIN_W: u32 = 600;
pub const WIN_H: u32 = 420;
use gfx::Device;
// import everything for prototyping
use conrod_core::*;
use conrod_core::position::*;

// App struct
pub struct DemoApp {
    ball_xy: conrod_core::Point,
    ball_color: conrod_core::Color,
    sine_frequency: f32,
    rust_logo: conrod_core::image::Id,
}

// App struct implementation
impl DemoApp {
    /// Simple constructor for the `DemoApp`.
    pub fn new(rust_logo: conrod_core::image::Id) -> Self {
        DemoApp {
            ball_xy: [0.0, 0.0],
            ball_color: conrod_core::color::WHITE,
            sine_frequency: 1.0,
            rust_logo: rust_logo,
        }
    }
}
// RGBA clear color 20% gray
const CLEAR_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];


// A wrapper around the winit window that allows us to implement the trait necessary for enabling
// the winit <-> conrod conversion functions.
struct WindowRef<'a>(&'a winit::Window);

// Implement the `WinitWindow` trait for `WindowRef` to allow for generating compatible conversion
// functions.
impl<'a> conrod_winit::WinitWindow for WindowRef<'a> {
    // get inner size
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        winit::Window::get_inner_size(&self.0).map(Into::into)
    }
    // hidpi factor
    fn hidpi_factor(&self) -> f32 {
        winit::Window::get_hidpi_factor(&self.0) as _
    }
}

// Generate the winit <-> conrod_core type conversion fns.
conrod_winit::conversion_fns!();

/// Uses assets folder, conrod, find_folder, gfx, glutin, winit
fn main() {
    // glutin::WindowBuilder constructor
    let builder = glutin::WindowBuilder::new()
        .with_title("tonal glyph tracker|demo")
        .with_dimensions((WIN_W, WIN_H).into());
    
    // winit::EventsLoop constructor
    let mut events_loop = winit::EventsLoop::new();

    // gfx things
    // DepthStencil
    type DepthFormat = gfx::format::DepthStencil;
    // glutin::ContextBuilder constructor
    let context = glutin::ContextBuilder::new()
        .with_multisampling(4);
    let (context, mut device, mut factory, rtv, _) =
        gfx_window_glutin::init::<conrod_gfx::ColorFormat, DepthFormat>(builder, context, &events_loop).unwrap();
    // gfx::Encoder
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    // conrod_gfx::Renderer
    let mut renderer = conrod_gfx::Renderer::new(&mut factory, &rtv, context.window().get_hidpi_factor() as f64).unwrap();

    // conrod_core::UiBuilder constructor
    let mut ui = conrod_core::UiBuilder::new([WIN_W as f64, WIN_H as f64])
        .theme(theme())
        .build();
    let ids = Ids::new(ui.widget_id_generator());

    // Load font from file
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // Load the Rust logo from our assets folder to use as an example image.
    fn load_rust_logo<T: gfx::format::TextureFormat,R: gfx_core::Resources, F: gfx::Factory<R>>(factory: &mut F) -> (gfx::handle::ShaderResourceView<R, <T as gfx::format::Formatted>::View>,(u32,u32)) {
        use gfx::{format, texture};
        use gfx::memory::Bind;
        use gfx::memory::Usage;
        let assets = find_folder::Search::ParentsThenKids(5, 3).for_folder("assets").unwrap();
        let path = assets.join("images/rust.png");
        let rgba_image = image::open(&std::path::Path::new(&path)).unwrap().to_rgba();
        let image_dimensions = rgba_image.dimensions();
        let kind = texture::Kind::D2(
            image_dimensions.0 as texture::Size,
            image_dimensions.1 as texture::Size,
            texture::AaMode::Single
        );
        let info = texture::Info {
            kind: kind,
            levels: 1,
            format: <T::Surface as format::SurfaceTyped>::get_surface_type(),
            bind: Bind::SHADER_RESOURCE,
            usage: Usage::Dynamic,
        };
        let raw = factory.create_texture_raw(
            info,
            Some(<T::Channel as format::ChannelTyped>::get_channel_type()),
            Some((&[rgba_image.into_raw().as_slice()], texture::Mipmap::Provided))).unwrap();
        // gfx_core::memory::Typed constructor
        let tex = gfx_core::memory::Typed::new(raw);
        let view = factory.view_texture_as_shader_resource::<T>(
            &tex, (0,0), format::Swizzle::new()
        ).unwrap();
        (view,image_dimensions)
    }
    // conrod_core::image::Map constructor
    let mut image_map = conrod_core::image::Map::new();
    let rust_logo = image_map.insert(load_rust_logo::<conrod_gfx::ColorFormat,_,_>(&mut factory));

    // DemoApp constructor
    let mut app = DemoApp::new(rust_logo);

    // main loop
    'main: loop {
        // If the window is closed, this will be None for one tick, so to avoid panicking with
        // unwrap, instead break the loop
        let (win_w, win_h): (u32, u32) = match context.window().get_inner_size() {
            Some(s) => s.into(),
            None => break 'main,
        };
        let dpi_factor = context.window().get_hidpi_factor() as f32;
        if let Some(primitives) = ui.draw_if_changed() {
            let dims = (win_w as f32 * dpi_factor, win_h as f32 * dpi_factor);

            //Clear the window
            renderer.clear(&mut encoder, CLEAR_COLOR);

            renderer.fill(&mut encoder,dims,dpi_factor as f64,primitives,&image_map);

            renderer.draw(&mut factory,&mut encoder,&image_map);

            encoder.flush(&mut device);
            context.swap_buffers().unwrap();
            device.cleanup();
        }
        let mut should_quit = false;
        events_loop.poll_events(|event|{
            // Convert winit event to conrod event, requires conrod to be built with the `winit` feature
            if let Some(event) = convert_event(event.clone(), &WindowRef(context.window())) {
                ui.handle_event(event);
            }
            // Close window if the escape key or the exit button is pressed
            match event {
                winit::Event::WindowEvent{event, .. } =>
                    match event {
                        winit::WindowEvent::KeyboardInput{ input: winit::KeyboardInput{ virtual_keycode: Some(winit::VirtualKeyCode::Escape),..}, ..} |
                        winit::WindowEvent::CloseRequested => should_quit = true,
                        winit::WindowEvent::Resized(logical_size) => {
                            let hidpi_factor = context.window().get_hidpi_factor();
                            let physical_size = logical_size.to_physical(hidpi_factor);
                            context.resize(physical_size);
                            let (new_color, _) = gfx_window_glutin::new_views::<conrod_gfx::ColorFormat, DepthFormat>(&context);
                            renderer.on_resize(new_color);
                        }
                        _ => {},
                    },
                _ => {},
            }
        });
        if should_quit {
            break 'main;
        }
        // Update widgets if any event has happened
        if ui.global_input().events().next().is_some() {
            let mut ui = ui.set_widgets();
            gui(&mut ui, &ids, &mut app);
        }
    }
}

/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod_core::Theme {
    conrod_core::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod_core::color::DARK_CHARCOAL,
        shape_color: conrod_core::color::LIGHT_CHARCOAL,
        border_color: conrod_core::color::BLACK,
        border_width: 0.0,
        label_color: conrod_core::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: conrod_core::theme::StyleMap::default(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}

/*
widgetids! {
    pub struct Ids {
        // The scrollable canvas
        canvas,
        canvas_scrollbar,
        instrument_canvas,
        pattern_canvas,
        scopes_canvas,
    }
}
*/

// Generate a unique `WidgetId` for each widget.
widget_ids! {
    pub struct Ids {
        // The scrollable canvas.
        canvas,
        // The title and introduction widgets.
        title,
        introduction,
        // Shapes.
        shapes_canvas,
        rounded_rectangle,
        shapes_left_col,
        shapes_right_col,
        shapes_title,
        line,
        point_path,
        rectangle_fill,
        rectangle_outline,
        trapezoid,
        oval_fill,
        oval_outline,
        circle,
        // Image.
        image_title,
        rust_logo,
        // Button, XyPad, Toggle.
        button_title,
        button,
        xy_pad,
        toggle,
        ball,
        // NumberDialer, PlotPath
        dialer_title,
        number_dialer,
        plot_path,
        // Scrollbar
        canvas_scrollbar,
    }
}


/// Instantiate a conrod GUI using DemoApp struct and a conrod UiCell
pub fn gui(ui: &mut conrod_core::UiCell, ids: &Ids, app: &mut DemoApp) {
    use std::iter::once;
    const MARGIN: conrod_core::Scalar = 30.0;
    const SHAPE_GAP: conrod_core::Scalar = 50.0;
    const TITLE_SIZE: conrod_core::FontSize = 42;
    const SUBTITLE_SIZE: conrod_core::FontSize = 32;
    // `Canvas` is a widget that provides some basic functionality for laying out children widgets.
    // By default, its size is the size of the window. We'll use this as a background for the
    // following widgets, as well as a scrollable container for the children widgets.
    const TITLE: &'static str = "Widgets";
    // conrod_core::widget::Canvas constructor
    widget::Canvas::new().pad(MARGIN).scroll_kids_vertically().set(ids.canvas, ui);

    ////////////////
    ///// TEXT /////
    ////////////////
    // We'll demonstrate the `Text` primitive widget by using it to draw a title and an
    // introduction to the example.
    // conrod_core::widget::Text constructor
    widget::Text::new(TITLE).font_size(TITLE_SIZE).mid_top_of(ids.canvas).set(ids.title, ui);
    const INTRODUCTION: &'static str =
        "This example aims to demonstrate all widgets that are provided by conrod.\
        \n\nThe widget that you are currently looking at is the Text widget. The Text widget \
        is one of several special \"primitive\" widget types which are used to construct \
        all other widget types. These types are \"special\" in the sense that conrod knows \
        how to render them via `conrod_core::render::Primitive`s.\
        \n\nScroll down to see more widgets!";
    // conrod_core::widget::Text constructor
    widget::Text::new(INTRODUCTION)
        .padded_w_of(ids.canvas, MARGIN)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .center_justify()
        .line_spacing(5.0)
        .set(ids.introduction, ui);

    ////////////////////////////
    ///// Lines and Shapes /////
    ////////////////////////////
    // conrod_core::widget::Text constructor
    widget::Text::new("Lines and Shapes")
        .down(70.0)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.shapes_title, ui);
    // Lay out the shapes in two horizontal columns.
    //
    // TODO: Have conrod provide an auto-flowing, fluid-list widget that is more adaptive for these
    // sorts of situations.
    // conrod_core::widget::Canvas constructor
    widget::Canvas::new()
        .down(0.0)
        .align_middle_x_of(ids.canvas)
        .kid_area_w_of(ids.canvas)
        .h(360.0)
        .color(conrod_core::color::TRANSPARENT)
        .pad(MARGIN)
        .flow_down(&[
            (ids.shapes_left_col, widget::Canvas::new()),
            (ids.shapes_right_col, widget::Canvas::new()),
        ])
        .set(ids.shapes_canvas, ui);
    let shapes_canvas_rect = ui.rect_of(ids.shapes_canvas).unwrap();
    let w = shapes_canvas_rect.w();
    let h = shapes_canvas_rect.h() * 5.0 / 6.0;
    let radius = 10.0;
    widget::RoundedRectangle::fill([w, h], radius)
        .color(conrod_core::color::CHARCOAL.alpha(0.25))
        .middle_of(ids.shapes_canvas)
        .set(ids.rounded_rectangle, ui);
    let start = [-40.0, -40.0];
    let end = [40.0, 40.0];
    widget::Line::centred(start, end).mid_left_of(ids.shapes_left_col).set(ids.line, ui);
    let left = [-40.0, -40.0];
    let top = [0.0, 40.0];
    let right = [40.0, -40.0];
    let points = once(left).chain(once(top)).chain(once(right));
    widget::PointPath::centred(points).right(SHAPE_GAP).set(ids.point_path, ui);
    widget::Rectangle::fill([80.0, 80.0]).right(SHAPE_GAP).set(ids.rectangle_fill, ui);
    widget::Rectangle::outline([80.0, 80.0]).right(SHAPE_GAP).set(ids.rectangle_outline, ui);
    let bl = [-40.0, -40.0];
    let tl = [-20.0, 40.0];
    let tr = [20.0, 40.0];
    let br = [40.0, -40.0];
    let points = once(bl).chain(once(tl)).chain(once(tr)).chain(once(br));
    widget::Polygon::centred_fill(points).mid_left_of(ids.shapes_right_col).set(ids.trapezoid, ui);
    widget::Oval::fill([40.0, 80.0]).right(SHAPE_GAP + 20.0).align_middle_y().set(ids.oval_fill, ui);
    widget::Oval::outline([80.0, 40.0]).right(SHAPE_GAP + 20.0).align_middle_y().set(ids.oval_outline, ui);
    widget::Circle::fill(40.0).right(SHAPE_GAP).align_middle_y().set(ids.circle, ui);

    /////////////////
    ///// Image /////
    /////////////////
    // conrod_core::widget::Text constructor
    widget::Text::new("Image")
        .down_from(ids.shapes_canvas, MARGIN)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.image_title, ui);
    const LOGO_SIDE: conrod_core::Scalar = 144.0;
    // conrod_core::widget::Image constructor
    widget::Image::new(app.rust_logo)
        .w_h(LOGO_SIDE, LOGO_SIDE)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .set(ids.rust_logo, ui);

    /////////////////////////////////
    ///// Button, XYPad, Toggle /////
    /////////////////////////////////
    // conrod_core::widget::Text constructor
    widget::Text::new("Button, XYPad and Toggle")
        .down_from(ids.rust_logo, 60.0)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.button_title, ui);
    let ball_x_range = ui.kid_area_of(ids.canvas).unwrap().w();
    let ball_y_range = ui.h_of(ui.window).unwrap() * 0.5;
    let min_x = -ball_x_range / 3.0;
    let max_x = ball_x_range / 3.0;
    let min_y = -ball_y_range / 3.0;
    let max_y = ball_y_range / 3.0;
    let side = 130.0;
    for _press in widget::Button::new()
        .label("PRESS ME")
        .mid_left_with_margin_on(ids.canvas, MARGIN)
        .down_from(ids.button_title, 60.0)
        .w_h(side, side)
        .set(ids.button, ui)
    {
        let x = rand::random::<conrod_core::Scalar>() * (max_x - min_x) - max_x;
        let y = rand::random::<conrod_core::Scalar>() * (max_y - min_y) - max_y;
        app.ball_xy = [x, y];
    }
    // conrod_core::widget::XYPad constructor
    for (x, y) in widget::XYPad::new(app.ball_xy[0], min_x, max_x,
                                     app.ball_xy[1], min_y, max_y)
        .label("BALL XY")
        .wh_of(ids.button)
        .align_middle_y_of(ids.button)
        .align_middle_x_of(ids.canvas)
        .parent(ids.canvas)
        .set(ids.xy_pad, ui)
    {
        app.ball_xy = [x, y];
    }
    let is_white = app.ball_color == conrod_core::color::WHITE;
    let label = if is_white { "WHITE" } else { "BLACK" };
    // conrod_core::widget::Toggle constructor
    for is_white in widget::Toggle::new(is_white)
        .label(label)
        .label_color(if is_white { conrod_core::color::WHITE } else { conrod_core::color::LIGHT_CHARCOAL })
        .mid_right_with_margin_on(ids.canvas, MARGIN)
        .align_middle_y_of(ids.button)
        .set(ids.toggle, ui)
    {
        app.ball_color = if is_white { conrod_core::color::WHITE } else { conrod_core::color::BLACK };
    }
    let ball_x = app.ball_xy[0];
    let ball_y = app.ball_xy[1] - max_y - side * 0.5 - MARGIN;
    widget::Circle::fill(20.0)
        .color(app.ball_color)
        .x_y_relative_to(ids.xy_pad, ball_x, ball_y)
        .set(ids.ball, ui);

    //////////////////////////////////
    ///// NumberDialer, PlotPath /////
    //////////////////////////////////
    // conrod_core::widget::Text constructor
    widget::Text::new("NumberDialer and PlotPath")
        .down_from(ids.xy_pad, max_y - min_y + side * 0.5 + MARGIN)
        .align_middle_x_of(ids.canvas)
        .font_size(SUBTITLE_SIZE)
        .set(ids.dialer_title, ui);
    // Use a `NumberDialer` widget to adjust the frequency of the sine wave below.
    let min = 0.5;
    let max = 200.0;
    let decimal_precision = 1;
    // conrod_core::widget::NumberDialer constructor
    for new_freq in widget::NumberDialer::new(app.sine_frequency, min, max, decimal_precision)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .w_h(160.0, 40.0)
        .label("F R E Q")
        .set(ids.number_dialer, ui)
    {
        app.sine_frequency = new_freq;
    }
    // Use the `PlotPath` widget to display a sine wave.
    let min_x = 0.0;
    let max_x = std::f32::consts::PI * 2.0 * app.sine_frequency;
    let min_y = -1.0;
    let max_y = 1.0;
    // conrod_core::widget::PlotPath constructor
    widget::PlotPath::new(min_x, max_x, min_y, max_y, f32::sin)
        .kid_area_w_of(ids.canvas)
        .h(240.0)
        .down(60.0)
        .align_middle_x_of(ids.canvas)
        .set(ids.plot_path, ui);

    /////////////////////
    ///// Scrollbar /////
    /////////////////////
    widget::Scrollbar::y_axis(ids.canvas).auto_hide(true).set(ids.canvas_scrollbar, ui);
}

