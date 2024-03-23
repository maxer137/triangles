use std::slice::Iter;

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
