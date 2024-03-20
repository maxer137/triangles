pub struct Vec2 {
    pub(crate) x: f64,
    pub(crate) y: f64,
}

impl From<(f64, f64)> for Vec2 {
    fn from(coords: (f64, f64)) -> Self {
        Vec2 { x: coords.0, y: coords.1 }
    }
}
