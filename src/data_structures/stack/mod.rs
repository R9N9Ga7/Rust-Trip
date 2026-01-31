#[allow(dead_code)]
pub struct Stack<T> {
    arr: Vec<T>,
}

#[allow(dead_code)]
impl<T> Stack<T> {
    pub fn new() -> Self {
        return Self { arr: Vec::new() };
    }

    pub fn push(&mut self, value: T) {
        self.arr.push(value);
    }

    pub fn pop(&mut self) -> Option<T> {
        return self.arr.pop();
    }

    pub fn top(&self) -> Option<&T> {
        return self.arr.last();
    }

    pub fn is_empty(&self) -> bool {
        return self.arr.is_empty();
    }

    pub fn size(&self) -> usize {
        return self.arr.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_i64() {
        let mut stack = Stack::<i64>::new();
        stack.push(2);
        assert_eq!(stack.arr[0], 2);
    }

    #[test]
    fn push_string() {
        let mut stack = Stack::<String>::new();
        stack.push("test".to_string());
        assert_eq!(stack.arr[0], "test".to_string());
    }

    #[test]
    fn pop_i64() {
        let mut stack = Stack::<i64>::new();
        stack.push(2);

        let value = stack.pop();

        assert_eq!(value, Some(2));
        assert!(stack.arr.is_empty())
    }

    #[test]
    fn pop_string() {
        let mut stack = Stack::<String>::new();
        stack.push("test".to_string());

        let value = stack.pop();

        assert_eq!(value, Some("test".to_string()));
        assert!(stack.arr.is_empty())
    }

    #[test]
    fn push_pop_order() {
        let mut stack = Stack::<i64>::new();
        stack.push(1);
        stack.push(2);

        {
            let value = stack.pop();
            assert_eq!(value, Some(2));
        }

        {
            let value = stack.pop();
            assert_eq!(value, Some(1));
        }
    }

    #[test]
    fn top_i64() {
        let mut stack = Stack::<i64>::new();
        stack.push(10);

        let value = stack.top().unwrap();
        assert_eq!(*value, 10);
        assert_eq!(stack.arr.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut stack = Stack::<i64>::new();

        assert!(stack.is_empty());

        stack.push(10);

        assert!(!stack.is_empty());

        stack.pop();

        assert!(stack.is_empty());
    }

    #[test]
    fn size() {
        let mut stack = Stack::<i64>::new();

        assert_eq!(stack.size(), 0);

        stack.push(10);
        stack.push(10);

        assert_eq!(stack.size(), 2);
    }
}
