pub struct WorkQueue<T: Clone> {
    data: Vec<T>,
    current_index: usize,
}

impl<T: Clone> WorkQueue<T> {
    pub fn create() -> WorkQueue<T> {
        WorkQueue::<T> {
            data: Vec::<T>::new(),
            current_index: 0,
        }
    }
    pub fn add(&mut self, item: T) {
        self.data.push(item);
    }
    pub fn current_value(&self) -> Option<T> {
        if self.current_index < self.data.len() {
            Some(self.data[self.current_index].clone())
        } else {
            None
        }
    }    
    pub fn current_ref(&self) -> Option<&T> {
        if self.current_index < self.data.len() {
            Some(&self.data[self.current_index])
        } else {
            None
        }
    }    
    pub fn next_value(&mut self) -> Option<T> {
        if self.current_index < self.data.len() {
            let result = Some(self.data[self.current_index].clone());
            self.current_index += 1;
            result
        } else {
            None
        }
    }
    pub fn next_ref(&mut self) -> Option<&T> {
        if self.current_index < self.data.len() {
            let result = Some(&self.data[self.current_index]);
            self.current_index += 1;
            result
        } else {
            None
        }
    }
    pub fn increment(&mut self) {
        self.current_index += 1;
    }
    pub fn clean(&mut self) {
        self.data = self.data[self.current_index..].to_vec();
        self.current_index = 0;
    }
    pub fn remaining(&self) -> usize {
        if self.data.len() > self.current_index {
            self.data.len() - self.current_index
        } else {
            0
        }
    }
    pub fn dirty_count(&self) -> usize {
        self.current_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let queue = WorkQueue::<i32>::create();
        assert_eq!(queue.remaining(), 0);
    }

    #[test]
    fn fill_queue() {
        let mut queue = WorkQueue::<i32>::create();
        queue.add(1);
        queue.add(2);
        queue.add(3);
        assert_eq!(queue.remaining(), 3);
    }

    #[test]
    fn next_queue() {
        let mut queue = WorkQueue::<i32>::create();
        let mut sum = 0;
        queue.add(1);
        queue.add(2);
        queue.add(3);

        while let Some(data) = queue.next_value() {
            sum += data;
            queue.clean();
        }

        assert_eq!(sum, 6);
    }

    #[test]
    fn in_loop_value_queue() {
        let mut queue = WorkQueue::<i32>::create();
        let mut sum = 0;
        queue.add(1);
        queue.add(2);
        queue.add(3);

        while let Some(data) = queue.next_value() {
            sum += data;
            if data == 3 {
                queue.add(4);
            }
        }

        assert_eq!(queue.remaining(), 0);
        assert_eq!(sum, 10);
    }
    #[test]
    fn in_loop_ref_queue() {
        let mut queue = WorkQueue::<i32>::create();
        let mut sum = 0;
        queue.add(1);
        queue.add(2);
        queue.add(3);

        while let Some(data) = queue.next_ref() {
            sum += data;
            if sum == 6 {
                queue.add(4);
            }
        }

        assert_eq!(queue.remaining(), 0);
        assert_eq!(sum, 10);
    }
    #[test]
    fn in_loop_not_clean() {
        let mut queue = WorkQueue::<i32>::create();
        let mut sum = 0;
        queue.add(1);
        queue.add(2);
        queue.add(3);

        while let Some(data) = queue.next_ref() {
            sum += data;
            if sum > 1 {
                assert_ne!(queue.dirty_count(), 0);
            }
        }

        assert_eq!(queue.remaining(), 0);
        assert_eq!(sum, 6);
    }    

    #[test]
    fn in_loop_clean() {
        let mut queue = WorkQueue::<i32>::create();
        let mut sum = 0;
        queue.add(1);
        queue.add(2);
        queue.add(3);

        while let Some(data) = queue.next_ref() {
            sum += data;
            queue.clean();
            assert_eq!(queue.dirty_count(), 0);
        }

        assert_eq!(queue.remaining(), 0);
        assert_eq!(sum, 6);
    }
}
