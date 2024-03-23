use std::ops::{Index, IndexMut};
use nannou::geom::Point2;
use crate::node::Node;
use std::vec::IntoIter;
use crate::edge::Edge;
use crate::tree_enum::TreesEnum;
use crate::tree_index::TreeIndex;


pub struct Tree {
    pub center: Node,
    pub tree1: Vec<Node>,
    pub tree2: Vec<Node>,
    pub tree3: Vec<Node>,
}

pub enum Triangle {
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
        //Get the basic start cycle
        let mut cycle = self.center_triangle();

        //Strict pass
        let mut strict = true;
        'strict: while cycle.len() != length {
            let prev_length = cycle.len();
            'cycle: for (prev, next) in (0..prev_length).zip((0..prev_length).cycle().skip(1)) {
                // if this is the strict pass the edges must be of the same edge
                if !cycle[prev].same_color(cycle[next]) && strict {
                    continue 'cycle;
                }
                // Get a list of all visible nodes from the first node
                let visible = self.check_node_vis_cycle_org(cycle[prev], &cycle);
                // For all of them
                'inner: for visible_node in visible {
                    match visible_node.makes_triangle((cycle[next], cycle[prev])) {
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
                    let other_vis = self.check_node_vis_cycle_org(cycle[next], &cycle);
                    if cycle.contains(&visible_node) {
                        continue 'inner;
                    }
                    if other_vis.contains(&visible_node) {
                        cycle.insert(next, visible_node);
                        break 'cycle;
                    }
                }
            }
            if cycle.len() == prev_length {
                if strict {
                    strict = false;
                    continue 'strict;
                } else {
                    return Err(());
                }
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

    pub fn find_smallest_triangle(&self) -> Vec<TreeIndex> {
        let mut all: Vec<TreeIndex> = self.check_node_vis(TreeIndex(TreesEnum::Center, 0));
        all.sort_by(|a, b | self[*a].dist(&self.center).partial_cmp(&self[*b].dist(&self.center)).unwrap());
        let mut output = vec![TreeIndex(TreesEnum::Center, 0)];
        let (mut f, mut s,mut t) = (false, false, false);
        for node in all {
            match node.0 {
                TreesEnum::Center => {}
                TreesEnum::First => {
                    if !f {
                        f = true;
                        output.push(node);
                    }
                }
                TreesEnum::Second => {
                    if !s {
                        s = true;
                        output.push(node);
                    }}
                TreesEnum::Third => {
                    if !t {
                        t = true;
                        output.push(node);
                    }}
            }
            if f && s && t {
                break;
            }
        }
        if !f || !s || !t {
            output = vec![]
        } else if self.verify_valid_4th(&output) {
            output.swap(0, 1);
        }
        output
    }

    pub fn verify_valid_4th(&self, cycle: &[TreeIndex]) -> bool {
        let edges: [Edge; 4] = [
            Edge(self[cycle[0]].pos, self[cycle[1]].pos),
            Edge(self[cycle[1]].pos, self[cycle[2]].pos),
            Edge(self[cycle[2]].pos, self[cycle[3]].pos),
            Edge(self[cycle[3]].pos, self[cycle[0]].pos),
        ];
        !(edges[0].intersects(edges[2]) || edges[1].intersects(edges[3]))
    }

    pub fn center_triangle(&self) -> Vec<TreeIndex> {
        let mut output = vec![
            TreeIndex(TreesEnum::Center, 0),
            TreeIndex(TreesEnum::First, 0),
            TreeIndex(TreesEnum::Second, 0),
            TreeIndex(TreesEnum::Third, 0)
        ];
        if !self.verify_valid_4th(&output) {
            output.swap(0, 1);
            if !self.verify_valid_4th(&output) {
                output.swap(1, 2);
            }
        }
        output
    }

    pub fn get_cycle_edges(&self, cycle: &[TreeIndex]) -> Vec<Edge> {
        let mut edges = vec![];
        for (prev, next) in (0..cycle.len()).zip((0..cycle.len()).cycle().skip(1)) {
            edges.push(Edge(self[cycle[prev]].pos, self[cycle[next]].pos));
        }
        edges
    }

    pub fn check_node_vis_cycle_org(&self, node_index: TreeIndex, cycle: &[TreeIndex]) -> Vec<TreeIndex> {
        let mut edges = self.get_cycle_edges(cycle);
        edges.append(&mut self.get_all_edges());
        self.check_node_vis_from_edge(node_index, edges)
    }

    pub fn check_node_vis_cycle(&self, node_index: TreeIndex, cycle: &[TreeIndex]) -> Vec<TreeIndex> {
        let edges = self.get_cycle_edges(cycle);
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
