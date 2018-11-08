#![allow(dead_code)]
#![allow(unused_imports)]
extern crate rutie; // Now we can use Ruby as a scripting language!!!
#[macro_use] extern crate conrod;
mod support;
use nn::*;
extern crate petgraph;
use std::collections::HashMap;
use nn::prelude as np; // nannou prelude
use nn::ui::prelude::*;
use nn::ui::prelude as uip; // ui prelude;
use nn::glium::*;
use nn::glium::{self, Surface};
use nn::ui::conrod::{self, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};
use nn::ui::conrod::widget::graph::{node, Event, EdgeEvent, Node, NodeEvent, NodeSocket};
use ui::widget::graph::Graph as NGraph;
use ui::widget::graph::Node as NNode;
use ui::widget::graph::Edge as NEdge;
widget_ids! {
    struct Ids {
        graph,
    }
}
type PGraph = petgraph::Graph<&'static str, (usize, usize)>;
type Layout = widget::graph::Layout<petgraph::graph::NodeIndex>;
pub fn main() {
    const WIDTH: u32 = 900;
    const HEIGHT: u32 = 500;
    let mut graph = PGraph::new();
    let a = graph.add_node("adc");
    let b = graph.add_node("dac");
    let c = graph.add_node("blackhole");
    let d = graph.add_node("ChucK VM");
    let e = graph.add_node("Faust stream");
    graph.extend_with_edges(&[
        (a, c, (1, 0)),
        (a, d, (0, 1)),
        (b, d, (0, 0)),
        (c, d, (0, 2)),
        (d, e, (0, 0)),
    ]);        
    let mut layout_map = HashMap::new(); // Construct a starting layout for the nodes.
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
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap(); // A type used for converting `conrod::render::Primitives` into `Command`s that can be used for drawing to the glium `Surface`.
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new(); // The image map describing each of our widget->image mappings (in our case, none).
    let mut event_loop = support::EventLoop::new();
    'main: loop {
        for event in event_loop.next(&mut events_loop) {
            if let Some(event) = conrod::backend::winit::convert_event(event.clone(), &display) {  // Use the `winit` backend feature to convert the winit event to a conrod one.
                ui.handle_event(event);
                event_loop.needs_update();
            }
            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => {  // Break from the loop upon `Escape` or closed window.
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
        if let Some(primitives) = ui.draw_if_changed() {  // Draw the `Ui` if it has changed.
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}  // The session is used in multiple stages:
fn set_widgets(ui: &mut conrod::UiCell, ids: &Ids, graph: &mut PGraph, layout: &mut Layout) {
    
    // 1. `Nodes` for setting a node widget for each node.
    // 2. `Edges` for setting an edge widget for each edge.
    // 3. `Final` for optionally displaying zoom percentage and cam position.
    let session = {
        let node_indices = graph.node_indices();  // An identifier for each node in the graph.
        let edges = graph.raw_edges()   // Describe each edge in the graph as NodeSocket -> NodeSocket.
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
    for event in session.events() {  // Graph events that have occurred since the last time the graph was instantiated.
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
    let mut session = session.next(); // Instantiate a widget for each node within the graph.
    for node in session.nodes() {
        // Each `Node` contains:
        // `id` - The unique node identifier for this node.
        // `point` - The position at which this node will be set.
        // `inputs`
        // `outputs`
        // Calling `node.widget(some_widget)` returns a `NodeWidget`, which contains:
        // `wiget_id` - The widget identifier for the widget that will represent this node.
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
    let mut session = session.next(); // Instantiate a widget for each edge within the graph.
    for edge in session.edges() {
        let (a, b) = node::edge_socket_rects(&edge, ui);
        let line = widget::Line::abs(a.xy(), b.xy())
            .color(conrod::color::DARK_CHARCOAL)
            .thickness(3.0);
        // Each edge contains:
        // `start` - The unique node identifier for the node at the start of the edge with point.
        // `end` - The unique node identifier for the node at the end of the edge with point.
        // `widget_id` - The wiget identifier for this edge.
        edge.widget(line).set(ui);
    }
}
