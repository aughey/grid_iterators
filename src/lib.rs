use std::collections::VecDeque;

trait DropQueue<T> {
    /// Returns true if it had to remove an element to insert the new one.
    fn insert(&mut self, value: T) -> bool;
    fn remove(&mut self) -> Option<T>;
}

/// Teach VecDeque to implement DropQueue.
impl<T> DropQueue<T> for VecDeque<T> {
    fn insert(&mut self, value: T) -> bool {
        if self.len() == self.capacity() {
            self.pop_front();
            self.push_back(value);
            true
        } else {
            self.push_back(value);
            false
        }
    }

    fn remove(&mut self) -> Option<T> {
        self.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::DropQueue;

    fn test_push_dropqueue<T>(queue: &mut impl DropQueue<T>)
    where
        T: Default,
    {
        assert!(!queue.insert(T::default()));
        assert!(!queue.insert(T::default()));
        assert!(!queue.insert(T::default()));
        assert!(!queue.insert(T::default()));
        assert!(!queue.insert(T::default()));
        assert!(queue.insert(T::default()));
    }

    #[test]
    fn it_works() {
        let mut queue = VecDeque::<i32>::with_capacity(5);

        test_push_dropqueue(&mut queue);
    }
}
