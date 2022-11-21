use std::cmp::max;
use std::rc::Rc;
use crate::error::{CResult, error};

/// A table of references to items of a kind.
pub struct Table<T> {
    name: &'static str,
    items: Vec<(Vec<String>, Rc<T>)>,
    longest_path: usize,
}

impl<T> Table<T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            items: vec![],
            longest_path: 0,
        }
    }

    /// Resolves the best match of the given path.
    /// When "Add" and "Op\Add" are available, "Add" should resolve to the former.
    /// In the "Op" scope, "Add" should resolve to the latter.
    /// When only "Op\Add" is available, "Add" should resolve to that.
    /// Only when the name is not available inside the scope, global search should be tried.
    pub fn resolve(&self, name: &str, scope: &str) -> CResult<Rc<T>> {
        let path = [Self::path(scope), Self::path(name)].concat();

        // Check shortest match first, then longer ones.
        for match_len in path.len()..=self.longest_path {
            for (item_path, item) in self.items.iter() {
                if item_path.len() == match_len {
                    let start = item_path.len() - match_len;
                    let shortened_item_path = &item_path[start..];

                    if shortened_item_path == &path {
                        return Ok(Rc::clone(item))
                    }
                }
            }
        }

        if scope != "" {
            // Retry without a scope.
            self.resolve(name, "")
        } else {
            panic!("No resolution for {} '{}'", self.name, name);

            error(format!("No resolution for {} {}", self.name, name), 0)
        }
    }

    pub fn declare(&mut self, name: &str, item: T) -> CResult<()> {
        let path = Self::path(name);

        if self.items.iter().any(|(p, _)| p == &path) {
            return error(format!("{} '{}' was declared twice", self.name, name), 0);
        }

        self.longest_path = max(self.longest_path, path.len());
        self.items.push((path, Rc::new(item)));

        Ok(())
    }

    fn path(string: &str) -> Vec<String> {
        string
            .split('\\')
            .map(|segment| segment.to_string())
            .collect()
    }
}
