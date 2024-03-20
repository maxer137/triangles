use std::ops::{Index, IndexMut};
use nannou::geom::Point2;
use crate::node::Node;
use std::slice::Iter;

pub struct Tree {
    pub pos: Point2,
    pub tree1: Vec<Node>,
    pub tree2: Vec<Node>,
    pub tree3: Vec<Node>,
}

impl Index<TreesEnum> for Tree {
    type Output = Vec<Node>;

    fn index(&self, tree: TreesEnum) -> &Self::Output {
        match tree {
            TreesEnum::First => {&self.tree1}
            TreesEnum::Second => {&self.tree2}
            TreesEnum::Third => {&self.tree3}
        }
    }
}

impl IndexMut<TreesEnum> for Tree {
    fn index_mut(&mut self, tree: TreesEnum) -> &mut Self::Output {
        match tree {
            TreesEnum::First => {&mut self.tree1}
            TreesEnum::Second => {&mut self.tree2}
            TreesEnum::Third => {&mut self.tree3}
        }
    }
}

#[derive(Copy, Clone)]
pub enum TreesEnum {
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

impl Tree {
    pub fn empty() -> Self {
        Self {
            pos: (0.0, 0.0).into(),
            tree1: vec![],
            tree2: vec![],
            tree3: vec![],
        }
    }
    
    fn get_branch_vector(&self, tree: TreesEnum) -> &Vec<Node> {
        match tree {
            TreesEnum::First => { &self.tree1 }
            TreesEnum::Second => { &self.tree2 }
            TreesEnum::Third => { &self.tree3 }
        }
    }

    pub fn find_node_at_pos(&self, pos: Point2, scale: f32) -> Option<(TreesEnum, usize)> {
        for tree in TreesEnum::iterator() {
            for (index, node) in self[*tree].iter().enumerate() {
                let dist = node.pos.distance(pos);
                if dist < scale {
                    return Some((*tree, index));
                }
            };
        }
        None
    }
    
    pub fn add_node(&mut self, tree: TreesEnum, node: Node) {
        self[tree].push(node);
    }
}
