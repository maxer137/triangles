use std::ops::{Index, IndexMut};
use nannou::geom::Point2;
use crate::node::Node;
use std::slice::Iter;
use std::vec::IntoIter;
use nannou::geom::rect::NUM_TRIANGLES;
use nannou::prelude::abs;

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

impl TreeIndex {
    pub fn same_color(&self, other: Self) -> bool {
        self.0 == other.0 || self.0 == TreesEnum::Center || other.0 == TreesEnum::Center
    }
    
    pub fn makes_triangle(&self, c2: Self, c3: Self) -> Triangle {
        if self.same_color(c2) && c2.same_color(c3) && c3.same_color(*self) {
            Triangle::AllSame
        } else if self.0 == c2.0 || c2.0 == c3.0 || c3.0 == self.0 {
            Triangle::OneOdd
        } else {
            Triangle::Illegal
        }
    }
}

enum Triangle {
    AllSame,
    OneOdd,
    Illegal
}

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

    pub fn find_cycle(&self, length: usize) -> Result<Vec<TreeIndex>, ()> {
        let mut cycle = self.start_cycle();
        let mut strict = true;
        'strict: while cycle.len() != length {
            let prev = cycle.len();
            let n = cycle.len();
            'cycle: for (prev, next) in (0..n).zip((0..n).cycle().skip(1)) {
                if !cycle[prev].same_color(cycle[next]) && strict {
                    continue 'cycle;
                }
                let visible = self.check_node_vis_cycle(cycle[prev], &cycle);
                'inner: for visible_node in visible {
                    match visible_node.makes_triangle(cycle[next], cycle[prev]) {
                        Triangle::AllSame => {}
                        Triangle::OneOdd => {
                            if strict {
                                continue 'inner;
                            }
                        }
                        Triangle::Illegal => {
                            continue 'inner;
                        }
                    }
                    let other_vis = self.check_node_vis_cycle(cycle[next], &cycle);
                    if cycle.contains(&visible_node) {
                        continue 'inner;
                    }
                    if other_vis.contains(&visible_node) {
                        cycle.insert(next, visible_node);
                        break 'cycle;
                    }
                }
            }
            if cycle.len() == prev {
                strict = false;
                continue 'strict;
            }
        }
        Ok(cycle)
    }

    pub fn start_cycle(&self) -> Vec<TreeIndex> {
        let c = TreeIndex(TreesEnum::Center, 0);
        let p1 = TreeIndex(TreesEnum::First, self.tree1.len()-1);
        let p2 = TreeIndex(TreesEnum::Second, self.tree2.len()-1);
        let p3 = TreeIndex(TreesEnum::Third, self.tree3.len()-1);
        vec![c, p1, p2, p3]
    }

    pub fn check_node_vis_cycle(&self, node_index: TreeIndex, cycle: &[TreeIndex]) -> Vec<TreeIndex> {
        let mut edges = vec![];
        for i in 0..cycle.len() - 1 {
            edges.push(Edge(self[cycle[i]].pos, self[cycle[i+1]].pos));
        }
        edges.push(Edge(self[*cycle.last().unwrap()].pos, self[*cycle.first().unwrap()].pos));
        edges.append(&mut self.get_all_edges());
        self.check_node_vis_from_edge(node_index, edges)
    }

    pub fn check_node_vis_from_edge(&self, node_index: TreeIndex, edges: Vec<Edge>) -> Vec<TreeIndex> {
        let mut output = vec![];
        'node: for index in self.iter() {
            if index == node_index {
                continue 'node;
            }
            let new_edge = Edge(self[node_index].pos, self[index].pos);
            'edge: for edge in &edges {
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

    pub fn check_node_vis(&self, node_index: TreeIndex) -> Vec<TreeIndex> {
        let all_edges = self.get_all_edges();
        self.check_node_vis_from_edge(node_index, all_edges)
    }
}
