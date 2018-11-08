#[macro_use] extern crate conrod;
mod support;
fn main() {
    feature::main();
}
mod feature {
    extern crate petgraph;
    use conrod::{self, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
    use conrod::backend::glium::glium::{self, Surface};
    use conrod::widget::graph::{node, Event, EdgeEvent, Node, NodeEvent, NodeSocket};
    use std::collections::HashMap;
    use support;
    widget_ids! {
        struct Ids {
            graph,
        }
    }
    type MyGraph = petgraph::Graph<&'static str, (usize, usize)>;
    type Layout = widget::graph::Layout<petgraph::graph::NodeIndex>;
    pub fn main() {
        const WIDTH: u32 = 900;
        const HEIGHT: u32 = 500;
        let mut graph = MyGraph::new();
        let a = graph.add_node("audio in");
        let b = graph.add_node("audio out");
        let c = graph.add_node("blackhole");
        let d = graph.add_node("instrument");
        let e = graph.add_node("plugin");
        graph.extend_with_edges(&[
            (a, c, (1, 0)),
            (a, d, (0, 1)),
            (b, d, (0, 0)),
            (c, d, (0, 2)),
            (d, e, (0, 0)),
        ]);
        let mut layout_map = HashMap::new();
        layout_map.insert(b, [-100.0, 100.0]);
        layout_map.insert(a, [-300.0, 0.0]);
        layout_map.insert(c, [-100.0, -100.0]);
        layout_map.insert(d, [100.0, 0.0]);
        layout_map.insert(e, [300.0, 0.0]);
        let mut layout = Layout::from(layout_map);
        let mut events_loop = glium::glutin::EventsLoop::new();
        let window = glium::glutin::WindowBuilder::new()
            .with_title("tgtracker|dsp graph")
            .with_dimensions((WIDTH, HEIGHT).into());
        let context = glium::glutin::ContextBuilder::new()
            .with_multisampling(4)
            .with_vsync(true);
        let display = glium::Display::new(window, context, &events_loop).unwrap();
        let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
        let ids = Ids::new(ui.widget_id_generator());
        const FONT_PATH: &'static str =
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/fonts/NotoSans/NotoSans-Regular.ttf");
        ui.fonts.insert_from_file(FONT_PATH).unwrap();
        let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();
        let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();
        let mut event_loop = support::EventLoop::new();
        'main: loop {
            // Handle all events.
            for event in event_loop.next(&mut events_loop) {
                // Use the `winit` backend feature to convert the winit event to a conrod one.
                if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {
                    ui.handle_event(event);
                    event_loop.needs_update();
                }
                // Break from the loop upon `Escape` or closed window.
                match event.clone() {
                    glium::glutin::Event::WindowEvent { event, .. } => {
                        match event {
                            glium::glutin::WindowEvent::CloseRequested |
                            glium::glutin::WindowEvent::KeyboardInput {
                                input: glium::glutin::KeyboardInput {
                                    virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                    ..
                                },
                                ..
                            } => break 'main,
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            set_widgets(&mut ui.set_widgets(), &ids, &mut graph, &mut layout);
            if let Some(primitives) = ui.draw_if_changed() {
                renderer.fill(&display, primitives, &image_map);
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 1.0);
                renderer.draw(&display, &mut target, &image_map).unwrap();
                target.finish().unwrap();
            }
        }
    }
    fn set_widgets(ui: &mut conrod::UiCell, ids: &Ids, graph: &mut MyGraph, layout: &mut Layout) {
        let session = {
            // An identifier for each node in the graph.
            let node_indices = graph.node_indices();
            // Describe each edge in the graph as NodeSocket -> NodeSocket.
            let edges = graph.raw_edges()
                .iter()
                .map(|e| {
                    let start = NodeSocket { id: e.source(), socket_index: e.weight.0 };
                    let end = NodeSocket { id: e.target(), socket_index: e.weight.1 };
                    (start, end)
                });
            widget::Graph::new(node_indices, edges, layout)
                .background_color(conrod::color::rgb(0.31, 0.33, 0.35))
                .wh_of(ui.window)
                .middle_of(ui.window)
                .set(ids.graph, ui)
        };
        for event in session.events() {
            match event {
                Event::Node(event) => match event {
                    // NodeEvent::Add(node_kind) => {
                    // },
                    NodeEvent::Remove(node_id) => {
                    },
                    NodeEvent::Dragged { node_id, to, .. } => {
                        *layout.get_mut(&node_id).unwrap() = to;
                    },
                },
                Event::Edge(event) => match event {
                    EdgeEvent::AddStart(node_socket) => {
                    },
                    EdgeEvent::Add { start, end } => {
                    },
                    EdgeEvent::Cancelled(node_socket) => {
                    },
                    EdgeEvent::Remove { start, end } => {
                    },
                },
            }
        }
        let mut session = session.next();
        for node in session.nodes() {
            let node_id = node.node_id();
            let inputs = graph.neighbors_directed(node_id, petgraph::Incoming).count();
            let outputs = graph.neighbors_directed(node_id, petgraph::Outgoing).count();
            let button = widget::Button::new()
                .label(&graph[node_id])
                .border(0.0);
            let widget = Node::new(button)
                .inputs(inputs)
                .outputs(outputs)
                .socket_color(conrod::color::LIGHT_RED)
                .w_h(100.0, 60.0);
            for _click in node.widget(widget).set(ui).widget_event {
                println!("{} was clicked!", &graph[node_id]);
            }
        }
        let mut session = session.next();
        for edge in session.edges() {
            let (a, b) = node::edge_socket_rects(&edge, ui);
            let line = widget::Line::abs(a.xy(), b.xy())
                .color(conrod::color::DARK_CHARCOAL)
                .thickness(3.0);
            edge.widget(line).set(ui);
        }
    }
}
