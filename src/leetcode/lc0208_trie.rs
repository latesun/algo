#[derive(Default)]
struct Trie {
    is_end: bool,
    children: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for c in word.bytes().map(|c| (c - b'a') as usize) {
            cur = cur.children[c].get_or_insert_with(|| Box::new(Trie::new()));
        }
        cur.is_end = true
    }

    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for c in word.bytes().map(|c| (c - b'a') as usize) {
            match &cur.children[c] {
                None => return false,
                Some(c) => cur = c,
            }
        }
        cur.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for c in prefix.bytes().map(|c| (c - b'a') as usize) {
            match &cur.children[c] {
                None => return false,
                Some(c) => cur = c,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trie() {
        let mut t = Trie::new();
        t.insert("hello".to_string());
        assert!(t.search("hello".to_string()));
        assert!(!t.search("he".to_string()));
        assert!(t.starts_with("he".to_string()));
    }
}
