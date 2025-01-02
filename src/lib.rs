// Traits - A way to teach the compiler how to implement a behavior for a type.
//        - A way to communicate to the caller what a type should do.
//        - A way to bound the impact a method will have on a value.

use std::collections::VecDeque;

trait DropQueue<T> {
    /// returns Some if it had to remove a value to insert the current value.
    fn insert(&mut self, value: T) -> Option<T>;
    fn remove(&mut self) -> Option<T>;
}

/// Teach VecDeque how to behave like a DropQueue
impl<T> DropQueue<T> for VecDeque<T> {
    fn insert(&mut self, value: T) -> Option<T> {
        if self.len() == self.capacity() {
            let ret = self.pop_front();
            self.push_back(value);
            ret
        } else {
            self.push_back(value);
            None
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

    fn test_dropqueue(queue: &mut impl DropQueue<i32>) {
        assert_eq!(queue.insert(1), None);
        assert_eq!(queue.insert(2), None);
        assert_eq!(queue.insert(3), None);
        assert_eq!(queue.insert(4), None);
        assert_eq!(queue.insert(5), None);
        assert_eq!(queue.insert(6), Some(1));
    }

    #[test]
    fn it_works() {
        let mut queue = VecDeque::<i32>::with_capacity(5);

        test_dropqueue(&mut queue);
    }
}
