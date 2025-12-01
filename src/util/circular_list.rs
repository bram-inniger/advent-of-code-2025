/// Circular doubly-linked list, backed by a never-shrinking Vector
/// To avoid the hassle of either `unsafe`
/// or a hell of `Rc<RefCell<Node<T>>>` and `Weak<RefCell<Node<T>>>`,
/// we simply store the backing Vector's indices.
#[derive(Debug, Default)]
pub struct CircularList<T> {
    nodes: Vec<Node<T>>,
    cursor: Option<usize>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    prev: usize,
    next: usize,
}

impl<T> CircularList<T> {
    /// Inserts a new node after the one the cursor points to (or inits the list if it was empty)
    /// This moves the cursor to the new node
    pub fn insert_clockwise(&mut self, el: T) {
        // We check the cursor to see if there exists a node already or not
        // If so, we adjust the existing node/nodes' pointers and move the cursor
        // If not, we create a new node and initialise the cursor to it
        if let Some(cursor) = self.cursor {
            let prev_idx = cursor;
            let next_idx = self.nodes[cursor].next;

            let node = Node {
                value: el,
                prev: prev_idx,
                next: next_idx,
            };
            self.nodes.push(node);

            let node_idx = self.nodes.len() - 1;

            self.nodes[prev_idx].next = node_idx;
            self.nodes[next_idx].prev = node_idx;

            self.cursor = Some(node_idx);
        } else {
            let node = Node {
                value: el,
                prev: 0,
                next: 0,
            };
            self.nodes.push(node);
            self.cursor = Some(self.nodes.len() - 1);
        }
    }

    pub fn rotate_clockwise(&mut self) {
        // In the edge case of an empty list (None cursor), rotation should be a no-op
        if let Some(cursor) = self.cursor {
            self.cursor = Some(self.nodes[cursor].next);
        }
    }

    pub fn rotate_clockwise_n(&mut self, n: u32) {
        (0..n).for_each(|_| self.rotate_clockwise());
    }

    pub fn rotate_counter_clockwise(&mut self) {
        // In the edge case of an empty list (None cursor), rotation should be a no-op
        if let Some(cursor) = self.cursor {
            self.cursor = Some(self.nodes[cursor].prev);
        }
    }

    pub fn rotate_counter_clockwise_n(&mut self, n: u32) {
        (0..n).for_each(|_| self.rotate_counter_clockwise());
    }
}

impl<T: Clone> CircularList<T> {
    /// Removes the current element at the index
    /// Panics if there are no elements in the list
    /// Advances the cursor clockwise, if there is a next element
    pub fn remove(&mut self) -> T {
        if let Some(cursor) = self.cursor {
            let node = &self.nodes[cursor];
            let prev_idx = node.prev;
            let next_idx = node.next;

            let value = node.value.clone();

            self.nodes[prev_idx].next = next_idx;
            self.nodes[next_idx].prev = prev_idx;

            // If cursor and the next index are the same,
            // it means this is the last element in the list that we remove
            self.cursor = if cursor == next_idx {
                None
            } else {
                Some(next_idx)
            };

            value
        } else {
            panic!("Cannot call `remove` on an empty list!")
        }
    }
}
