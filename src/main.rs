mod tree;
mod node;
mod renderer;

use nannou::App;
use nannou::geom::Point2;
use crate::node::Node;
use crate::renderer::{event, Model, RenderOptions, view};
use crate::tree::{Edge, Tree, TreesEnum};


pub fn model(_app: &App) -> Model {
    let mut t = Tree::empty();
    for i in 0..10 {
        t.add_node(TreesEnum::First, Node::from_pos(-10.0, (i * 25) as f32));
        t.add_node(TreesEnum::Second, Node::from_pos(-10.0 - (i * 20) as f32, -25.0 -(i * 25) as f32));
        t.add_node(TreesEnum::Third, Node::from_pos(10.0 + (i * 20) as f32, -(i * 25) as f32));
    }
    t.add_node(TreesEnum::Second, Node::from_pos(-250.0, -300.0));
    t.add_node(TreesEnum::Third, Node::from_pos(250.0, -300.0));
    Model {
        tree: t,
        camera: Point2::new(0.0, 200.0),
        scale: 1.0,
        selected: None,
        click: (0.0, 0.0).into(),
        render_options: RenderOptions::default(),
    }
}

fn main() {
    let e1 = Edge(Point2::new(-1.0, 0.0), Point2::new(1.0, 0.0));
    let e2 = Edge(Point2::new(0.0, -1.0), Point2::new(0.0, 1.0));
    println!("{}", e1.intersects(e2));
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

