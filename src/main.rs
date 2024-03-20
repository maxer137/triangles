mod tree;
mod vec;
mod node;
mod renderer;

use crate::renderer::{event, model, view};


fn main() {
    nannou::app(model)
        .event(event)
        .simple_window(view)
        .run();
}

