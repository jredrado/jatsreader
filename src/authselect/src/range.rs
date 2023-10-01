

use indextree::{NodeId,Arena,NodeEdge};

#[derive(Clone)]
/// An iterator of the "sides" of a node visited during a depth-first pre-order traversal,
/// where node sides are visited start to end and children are visited in insertion order.
///
/// i.e. node.start -> first child -> second child -> node.end
pub struct Range<'a, T> {
    arena: &'a Arena<T>,
    root: NodeId,
    from: NodeId,
    to: NodeId,
    next: Option<NodeEdge>,
}

impl<'a, T> Range<'a, T> {
    pub(crate) fn new(arena: &'a Arena<T>, root: NodeId,from: NodeId, to:NodeId) -> Self {
        Self {
            arena,
            root: root,
            from: from,
            to: to,
            next: Some(NodeEdge::Start(from)),
        }
    }

    /// Calculates the next node.
    fn next_of_next(&self, next: NodeEdge) -> Option<NodeEdge> {
        match next {
            NodeEdge::Start(node) => match self.arena[node].first_child() {
                Some(first_child) => Some(NodeEdge::Start(first_child)),
                None => Some(NodeEdge::End(node)),
            },
            NodeEdge::End(node) => {
                if node == self.root || node == self.to {
                    return None;
                }
                let node = &self.arena[node];
                match node.next_sibling() {
                    Some(next_sibling) => Some(NodeEdge::Start(next_sibling)),
                    // `node.parent()` here can only be `None` if the tree has
                    // been modified during iteration, but silently stoping
                    // iteration seems a more sensible behavior than panicking.
                    None => node.parent().map(NodeEdge::End),
                }
            }
        }
    }
}

impl<'a, T> Iterator for Range<'a, T> {
    type Item = NodeEdge;

    fn next(&mut self) -> Option<NodeEdge> {
        let next = self.next.take()?;
        self.next = self.next_of_next(next);
        Some(next)
    }
}