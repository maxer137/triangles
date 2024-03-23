use nannou::geom::Point2;

#[derive(Copy, Clone)]
pub struct Edge(pub Point2, pub Point2);

impl Edge {
    pub fn intersects(&self, e: Edge) -> bool {
        let p = self.0;
        let q = e.0;
        let r = Point2::new(self.1.x - self.0.x, self.1.y - self.0.y);
        let s = Point2::new(e.1.x - e.0.x, e.1.y - e.0.y);
        let r_cross_s = r.x * s.y - r.y * s.x;
        let q_minus_p = Point2::new(q.x - p.x, q.y - p.y);
        let q_minus_p_cross_r = q_minus_p.x * r.y - q_minus_p.y * r.x;

        if r_cross_s == 0.0 {
            // Parallel lines
            if q_minus_p_cross_r == 0.0 {
                // Collinear lines
                let t0 = (q.x - p.x) / r.x;
                let t1 = (q.y - p.y) / r.y;
                // Check if the ranges of t overlap
                return (0.0..=1.0).contains(&t0) || (0.0..=1.0).contains(&t1);
            }
            return false;
        }

        let t = (q_minus_p_cross_r) / r_cross_s;
        let u = (q_minus_p.x * s.y - q_minus_p.y * s.x) / r_cross_s;

        (0.0..=1.0).contains(&t) && (0.0..=1.0).contains(&u)
    }
}
