use crate::tree::Triangle;
use crate::tree_enum::TreesEnum;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TreeIndex(pub TreesEnum, pub usize);

impl TreeIndex {
    pub fn same_color(&self, other: Self) -> bool {
        self.0 == other.0 || self.0 == TreesEnum::Center || other.0 == TreesEnum::Center
    }

    pub fn makes_triangle(&self, edge: (Self, Self)) -> Triangle {
        if self.same_color(edge.0) && self.same_color(edge.1) && edge.1.same_color(*self) {
            Triangle::AllSame
        } else if self.same_color(edge.0) || self.same_color(edge.1) {
            Triangle::OneOdd
        } else {
            Triangle::Illegal
        }
    }
}
