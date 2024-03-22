mod tree;
mod node;
mod renderer;

use nannou::geom::Point2;
use crate::renderer::{event, model, view};
use crate::tree::Edge;


fn main() {
    let e1 = Edge(Point2::new(-1.0, 0.0), Point2::new(1.0, 0.0));
    let e2 = Edge(Point2::new(0.0, -1.0), Point2::new(0.0, 1.0));
    println!("{}", e1.intersects(e2));
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

