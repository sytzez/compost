use crate::error::{error, CResult, ErrorMessage};
use std::cmp::max;
use std::rc::Rc;

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
            for (item_path, item) in &self.items {
                if item_path.len() == match_len {
                    let start = item_path.len() - path.len();
                    let shortened_item_path = &item_path[start..];

                    if shortened_item_path == path {
                        return Ok(Rc::clone(item));
                    }
                }
            }
        }

        if !scope.is_empty() {
            // Retry without a scope.
            self.resolve(name, "")
        } else {
            error(ErrorMessage::NoResolution(self.name, name.into()))
        }
    }

    /// Resolves all matches within a given path.
    pub fn resolve_wildcard(&self, name: &str) -> CResult<Vec<Rc<T>>> {
        let path = Self::path(name);
        let len = path.len();

        let mut result = vec![];
        for (item_path, item) in &self.items {
            if item_path.len() > len && item_path[0..len] == path {
                result.push(Rc::clone(item))
            }
        }

        Ok(result)
    }

    pub fn declare(&mut self, name: &str, item: T) -> CResult<Rc<T>> {
        let path = Self::path(name);

        if self.items.iter().any(|(p, _)| p == &path) {
            return error(ErrorMessage::DoubleDeclaration(self.name, name.into()));
        }

        let rc = Rc::new(item);

        self.longest_path = max(self.longest_path, path.len());
        self.items.push((path, Rc::clone(&rc)));

        Ok(rc)
    }

    fn path(string: &str) -> Vec<String> {
        string
            .split('\\')
            .filter(|segment| !segment.is_empty())
            .map(|segment| segment.to_string())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use crate::sem::table::Table;

    #[test]
    fn test() {
        let mut table = Table::new("Integer");

        table.declare("Mod\\Thing", 1).unwrap();
        table.declare("Mod", 2).unwrap();

        assert_eq!(table.resolve("Thing", "").unwrap().as_ref(), &1);
        assert_eq!(table.resolve("Mod", "").unwrap().as_ref(), &2);
    }
}
