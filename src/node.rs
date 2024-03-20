use nannou::geom::Point2;


#[derive(Debug)]
pub struct Node {
    pub pos: Point2,
}


impl Node {
    pub fn default() -> Self {
        Node {pos: (0.0, 0.0).into()}
    }

    pub fn from_pos(x: f32, y: f32) -> Self {
        Node {pos: Point2::new(x, y)}
    }
}

