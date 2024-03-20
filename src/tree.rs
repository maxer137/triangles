use nannou::geom::Point2;
use crate::node::Node;

pub struct Tree {
    pub pos: Point2,
    pub tree1: Vec<Node>,
    pub tree2: Vec<Node>,
    pub tree3: Vec<Node>,
}

pub enum TreesEnum {
    First,
    Second,
    Third,
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

    pub fn get_tree_leaf(&self, tree: TreesEnum) -> Option<&Node> {
        let node_ref = self.get_branch_vector(tree);
        node_ref.last()
    }
    
    fn get_branch_vector(&self, tree: TreesEnum) -> &Vec<Node> {
        match tree {
            TreesEnum::First => { &self.tree1 }
            TreesEnum::Second => { &self.tree2 }
            TreesEnum::Third => { &self.tree3 }
        }
    }

    pub fn find_node_at_pos(&self, pos: Point2, scale: f32) -> Option<(TreesEnum, usize)> {
        for (index, node) in self.tree1.iter().enumerate() {
            let dist = node.pos.distance(pos);
            if dist < scale {
                return Some((TreesEnum::First, index));
            }
        };
        for (index, node) in self.tree2.iter().enumerate() {
            let dist = node.pos.distance(pos);
            if dist < scale {
                return Some((TreesEnum::Second, index));
            }
        };
        for (index, node) in self.tree3.iter().enumerate() {
            let dist = node.pos.distance(pos);
            if dist < scale {
                return Some((TreesEnum::Third, index));
            }
        };
        None
    }
    
    fn get_mut_branch_vector(&mut self, tree: TreesEnum) -> &mut Vec<Node> {
        match tree {
            TreesEnum::First => { &mut self.tree1 }
            TreesEnum::Second => { &mut self.tree2 }
            TreesEnum::Third => { &mut self.tree3 }
        }
    }

    pub fn get_leaves(&self) -> [Option<&Node>; 3] {
        [
            self.get_tree_leaf(TreesEnum::First),
            self.get_tree_leaf(TreesEnum::Second),
            self.get_tree_leaf(TreesEnum::Third)
        ]
    }
    
    pub fn add_node(&mut self, tree: TreesEnum, node: Node) {
        let vector: &mut Vec<Node> = self.get_mut_branch_vector(tree);
        vector.push(node);
    }
}
