use nannou::geom::Point2;


#[derive(Debug, Copy, Clone)]
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
    
    pub fn dist(&self, other: &Self) -> f32 {
        self.pos.distance(other.pos)
    }
}

