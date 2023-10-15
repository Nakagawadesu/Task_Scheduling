use std::cmp::Ordering;
use petgraph::stable_graph::NodeIndex;
#[derive(Debug, PartialEq)]

pub(crate) struct TaskTuple {
    pub(crate) node: NodeIndex,
    pub(crate) priority: f64,
}

impl Eq for TaskTuple {}

impl Ord for TaskTuple {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.partial_cmp(&other.priority).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for TaskTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TaskTuple {
    pub fn new(new_node: NodeIndex, priority_value: f64) -> Self {
        Self {
            node: new_node,
            priority: priority_value,
        }
    }
}