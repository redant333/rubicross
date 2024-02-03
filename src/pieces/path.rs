use bezier_rs::{Identifier, Subpath};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct EmptyId;

impl Identifier for EmptyId {
    fn new() -> Self {
        Self
    }
}

pub type SubpathNoId = Subpath<EmptyId>;

pub struct Path {
    pub main_path: SubpathNoId,
    pub ghost_path: Option<SubpathNoId>,
}
