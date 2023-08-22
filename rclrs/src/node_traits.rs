use crate::node::Node;
use std::sync::Arc;

pub trait ComponentNode {
    fn node(&self) -> &Node;
    fn node(&self) -> &Arc<Node>;
}
