use std::collections::HashMap;
use std::rc::Rc;
use crate::error::{CResult, error};

type ReferencePath = Vec<String>;

fn path(string: &str) -> ReferencePath {
    string
        .split('\\')
        .map(|segment| segment.to_string())
        .collect()
}

/// A table of references to items of a kind.
pub struct Table<T> {
    items: HashMap<ReferencePath, Rc<T>>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self { items: HashMap::new() }
    }

    pub fn resolve(&self, name: &str) -> CResult<Rc<T>> {
        let path = path(name);
        let end = path.len() - 1;

        for i in 0..=end {
            let partial_path = &path[(end - i)..end];

            if let Some(item) = self.items.get(partial_path) {
                return Ok(Rc::clone(item))
            }
        }

        error(format!("No resolution for {}", name), 0)
    }

    pub fn declare(&mut self, name: &str, item: T) -> CResult<()> {
        match self.items.insert(path(name), Rc::new(item)) {
            None => Ok(()),
            Some(_) => error(format!("{} is already declared", name), 0),
        }
    }
}
