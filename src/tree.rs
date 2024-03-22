use std::ops::{Index, IndexMut};
use nannou::geom::Point2;
use crate::node::Node;
use std::slice::Iter;

pub struct Tree {
    pub center: Node,
    pub tree1: Vec<Node>,
    pub tree2: Vec<Node>,
    pub tree3: Vec<Node>,
}

pub struct TreeIndex(TreesEnum, usize);

impl Tree {
    pub fn iter(&self) -> Iter<'static, TreeIndex> {
        let mut output = vec![TreeIndex(TreesEnum::Center, 0)];
        for branch in TreesEnum::iterator() {
            for i in 0..self[*branch].len() {
                output.push(TreeIndex(*branch, i))
            }
        }
        output.iter()
    }
}


impl Index<TreeIndex> for Tree {
    type Output = Node;
    fn index(&self, index: TreeIndex) -> &Self::Output {
        &self[index.0][index.1]
    }
}

impl IndexMut<TreeIndex> for Tree {
    fn index_mut(&mut self, index: TreeIndex) -> &mut Self::Output {
        &mut self[index.0][index.1]
    }
}

#[derive(Copy, Clone)]
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
            _ => { &vec![] }
        }
    }
}

impl IndexMut<TreesEnum> for Tree {
    fn index_mut(&mut self, tree: TreesEnum) -> &mut Self::Output {
        match tree {
            TreesEnum::First => { &mut self.tree1 }
            TreesEnum::Second => { &mut self.tree2 }
            TreesEnum::Third => { &mut self.tree3 }
            _ => { &mut vec![] }
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

    pub fn find_special_nodes(&self) -> Vec<TreeIndex> {
        vec![]
    }
}
