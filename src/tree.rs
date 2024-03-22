use std::ops::{Index, IndexMut};
use nannou::geom::Point2;
use crate::node::Node;
use std::slice::Iter;
use std::vec::IntoIter;

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

        t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0
    }
}


pub struct Tree {
    pub center: Node,
    pub tree1: Vec<Node>,
    pub tree2: Vec<Node>,
    pub tree3: Vec<Node>,
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TreeIndex(pub TreesEnum, pub usize);

impl Tree {
    pub fn iter(&self) -> IntoIter<TreeIndex> {
        let mut output = vec![TreeIndex(TreesEnum::Center, 0)];
        for branch in TreesEnum::iterator() {
            for i in 0..self[*branch].len() {
                output.push(TreeIndex(*branch, i))
            }
        }
        output.into_iter()
    }
}


impl Index<TreeIndex> for Tree {
    type Output = Node;
    fn index(&self, index: TreeIndex) -> &Self::Output {
        if let TreesEnum::Center = index.0 {
            return &self.center;
        }
        &self[index.0][index.1]
    }
}

impl IndexMut<TreeIndex> for Tree {
    fn index_mut(&mut self, index: TreeIndex) -> &mut Self::Output {
        if let TreesEnum::Center = index.0 {
            return &mut self.center;
        }
        &mut self[index.0][index.1]
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum TreesEnum {
    Center,
    First,
    Second,
    Third,
}

impl TreesEnum {
    pub fn iterator() -> Iter<'static, TreesEnum> {
        static DIRECTIONS: [TreesEnum; 3] = [TreesEnum::First, TreesEnum::Second, TreesEnum::Third];
        DIRECTIONS.iter()
    }
}

impl Index<TreesEnum> for Tree {
    type Output = Vec<Node>;

    fn index(&self, tree: TreesEnum) -> &Self::Output {
        match tree {
            TreesEnum::First => { &self.tree1 }
            TreesEnum::Second => { &self.tree2 }
            TreesEnum::Third => { &self.tree3 }
            _ => { &self.tree1 }
        }
    }
}

impl IndexMut<TreesEnum> for Tree {
    fn index_mut(&mut self, tree: TreesEnum) -> &mut Self::Output {
        match tree {
            TreesEnum::First => { &mut self.tree1 }
            TreesEnum::Second => { &mut self.tree2 }
            TreesEnum::Third => { &mut self.tree3 }
            _ => { &mut self.tree1 }
        }
    }
}

impl Tree {
    pub fn empty() -> Self {
        Self {
            center: Node::default(),
            tree1: vec![],
            tree2: vec![],
            tree3: vec![],
        }
    }
    pub fn find_node_at_pos(&self, pos: Point2, scale: f32) -> Option<TreeIndex> {
        for index in self.iter() {
            let dist = self[index].pos.distance(pos);
            if dist < scale {
                return Some(index);
            }
        }
        None
    }

    pub fn add_node(&mut self, tree: TreesEnum, node: Node) {
        self[tree].push(node);
    }

    pub fn get_all_edges(&self) -> Vec<Edge> {
        let mut output = vec![];
        for tree_branch in TreesEnum::iterator() {
            output.append(&mut self.get_tree_edges(*tree_branch))
        }
        output
    }

    pub fn get_tree_edges(&self, branch: TreesEnum) -> Vec<Edge> {
        let mut output = vec![];
        for i in 0..self[branch].len() - 1 {
            // Print the current element and the next element
            output.push(Edge(self[branch][i].pos, self[branch][i + 1].pos));
        }
        output.push(Edge(self[branch].first().unwrap().pos, self.center.pos));
        output
    }

    pub fn find_special_nodes(&self) -> Vec<TreeIndex> {
        let mut output = vec![];
        for node in self.iter() {
            if self.check_node_vis(node).len() == 3 {
                output.push(node);
            }
        }
        output
    }


    pub fn check_node_vis(&self, node_index: TreeIndex) -> Vec<TreeIndex> {
        let mut output = vec![];
        let all_edges = self.get_all_edges();
        'node: for index in self.iter() {
            if index == node_index {
                continue 'node;
            }
            let new_edge = Edge(self[node_index].pos, self[index].pos);
            'edge: for edge in &all_edges {
                if edge.0 == self[node_index].pos || edge.1 == self[node_index].pos {
                    continue 'edge;
                }
                if edge.0 == self[index].pos || edge.1 == self[index].pos {
                    continue 'edge;
                }
                if new_edge.intersects(*edge) {
                    continue 'node;
                }
            }
            output.push(index)
        }
        output
    }
}
