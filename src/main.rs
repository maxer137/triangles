mod tree;
mod node;
mod renderer;
mod edge;
mod tree_index;
mod tree_enum;

use std::f32::consts::SQRT_2;
use nannou::App;
use nannou::geom::Point2;
use tree_enum::TreesEnum;
use crate::node::Node;
use crate::renderer::{event, Model, RenderOptions, view};
use crate::tree::Tree;


pub fn model(_app: &App) -> Model {
    let mut t = Tree::empty();
    let total = 6;
    for i in 0..total {
        let dist = 200.0 / total as f32;
        t.add_node(TreesEnum::First, Node::from_pos(0.0, -dist - (i as f32 * dist)));
        t.add_node(TreesEnum::Second, Node::from_pos(-dist - (i as f32 * dist) / SQRT_2, dist + (i as f32 * dist) / SQRT_2));
        t.add_node(TreesEnum::Third, Node::from_pos(dist + (i as f32 * dist) / SQRT_2, dist + (i as f32 * dist) / SQRT_2));
    }
    let cycle_len = t.iter().len();
    Model {
        tree: t,
        camera: Point2::new(0.0, 200.0),
        scale: 1.0,
        selected: None,
        click: (0.0, 0.0).into(),
        cycle_len,
        render_options: RenderOptions::default(),
    }
}

fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

