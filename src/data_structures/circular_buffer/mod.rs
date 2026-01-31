#[allow(dead_code)]
struct CircularBuffer<T, const N: usize> {
    buffer: [Option<T>; N],
    write_index: usize,
    read_index: usize,
    count: usize,
}

#[allow(dead_code)]
impl<T, const N: usize> CircularBuffer<T, N> {
    fn new() -> Self {
        assert!(N > 0, "CircularBuffer must be > 0");
        Self {
            buffer: std::array::from_fn(|_| None),
            read_index: 0,
            write_index: 0,
            count: 0,
        }
    }

    fn put(&mut self, value: T) -> bool {
        if self.is_full() {
            return false;
        }

        self.buffer[self.write_index] = Some(value);
        self.write_index = (self.write_index + 1) % N;
        self.count += 1;
        true
    }

    fn get(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let value = self.buffer[self.read_index].take();
        self.count -= 1;
        self.read_index = (self.read_index + 1) % N;
        value
    }

    fn is_empty(&self) -> bool { self.count == 0 }
    fn is_full(&self) -> bool { self.count == N }
    fn capacity(&self) -> usize { self.count }
    fn len(&self) -> usize { N }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_i64() {
        let mut buffer = CircularBuffer::<i64, 2>::new();

        assert!(buffer.put(10));
        assert_eq!(buffer.buffer[0], Some(10));
        assert_eq!(buffer.write_index, 1);
        assert_eq!(buffer.read_index, 0);
    }

    #[test]
    fn put_string() {
        let mut buffer = CircularBuffer::<String, 2>::new();

        assert!(buffer.put("test".to_string()));
        assert_eq!(buffer.buffer[0], Some("test".to_string()));
        assert_eq!(buffer.write_index, 1);
        assert_eq!(buffer.read_index, 0);
    }

    #[test]
    fn put_with_overflow() {
        let mut buffer = CircularBuffer::<i64, 2>::new();
        assert!(buffer.put(1));
        assert!(buffer.put(2));
        assert!(!buffer.put(3));
    }

    #[test]
    fn get_i64() {
        let mut buffer = CircularBuffer::<i64, 2>::new();
        buffer.put(1);
        buffer.put(2);
        assert_eq!(buffer.get(), Some(1));
        assert_eq!(buffer.get(), Some(2));
        assert_eq!(buffer.get(), None);
    }

    #[test]
    fn get_string() {
        let mut buffer = CircularBuffer::<String, 2>::new();
        buffer.put("test-1".to_string());
        buffer.put("test-2".to_string());
        assert_eq!(buffer.get(), Some("test-1".to_string()));
        assert_eq!(buffer.get(), Some("test-2".to_string()));
        assert_eq!(buffer.get(), None);
    }

    #[test]
    fn circular_movement() {
        let mut buffer = CircularBuffer::<i64, 2>::new();
        buffer.put(1);
        buffer.put(2);
        buffer.get();
        buffer.put(3);

        assert_eq!(buffer.buffer[0], Some(3));
        assert_eq!(buffer.buffer[1], Some(2));
    }
}
