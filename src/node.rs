#[derive(Debug)]
pub struct Node {
    pub value: Option<char>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: Option<char>) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
    fn eq(&self, other: &Node) -> bool {
        if self.value != other.value {
            return false;
        }
        if self.left.is_some() && other.left.is_some() {
            if !self.left.as_ref().unwrap().eq(other.left.as_ref().unwrap()) {
                return false;
            }
        } else if self.left.is_some() || other.left.is_some() {
            return false;
        }
        if self.right.is_some() && other.right.is_some() {
            if !self.right.as_ref().unwrap().eq(other.right.as_ref().unwrap()) {
                return false;
            }
        } else if self.right.is_some() || other.right.is_some() {
            return false;
        }
        true
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.eq(other)
    }
}