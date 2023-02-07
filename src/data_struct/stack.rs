#[derive(Debug)]
struct Stack<T> {
    top: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: 0,
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, data: T) {
        self.top += 1;
        self.data.push(data);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            return None;
        }

        self.top -= 1;
        self.data.pop()
    }

    pub fn peak(&self) -> Option<&T> {
        if self.top == 0 {
            return None;
        }
        self.data.get(self.top - 1)
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack() {
        let mut s = Stack::new();
        assert!(s.is_empty());

        s.push(1);
        s.push(2);
        s.push(3);
        s.pop();
        if let Some(peak) = s.peak() {
            assert_eq!(2, *peak);
        }
    }
}
