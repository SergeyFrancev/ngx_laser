pub struct UrlTree {
    parts: Vec<String>,
    parent: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
}

impl UrlTree {
    pub fn new() -> UrlTree {
        UrlTree {
            parts: Vec::from([String::from("/")]),
            parent: Vec::from([None]),
            children: Vec::from([Vec::from([])]),
        }
    }

    pub fn add(&mut self, url: &str) -> usize {
        let path = url.split('?').next().unwrap();
        let mut parent = 0;
        for part in path.split('/').skip(1) {
            let child = self.find_child(part, parent);
            if child.is_none() {
                let key = self.parts.len();
                self.parts.push(part.to_string());
                self.parent.push(Some(parent));
                self.children.push(Vec::from([]));
                parent = key;
            } else {
                parent = child.unwrap();
            }
        }
        parent
    }

    fn find_child(&self, part: &str, parent: usize) -> Option<usize> {
        for child in self.children[parent].iter() {
            if self.parts[*child] == part {
                return Some(*child);
            }
        }
        None
    }
}
