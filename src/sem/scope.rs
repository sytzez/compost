use std::backtrace::Backtrace;
use crate::error::{error, CResult};
use std::rc::Rc;

pub type ReferencePath = Vec<String>;

pub fn path(string: &str) -> ReferencePath {
    string
        .split('\\')
        .map(|segment| segment.to_string())
        .collect()
}

// pub struct Scope {
//     traits: Table<Trait>,
//     classes: Table<Class>,
//     structs: Table<Struct>,
//     lets: Table<Let>,
//     defs: Table<Definition>,
// }
//
// pub struct LocalScope<'a> {
//     locals: HashMap<String, Rc<Instance>>,
//     zelf: Option<Rc<Instance>>,
//     scope: &'a Scope,
// }
//
// impl Scope {
//     pub fn new() -> Self {
//         Self {
//             traits: Table::new(),
//             classes: Table::new(),
//             structs: Table::new(),
//             lets: Table::new(),
//             defs: Table::new(),
//         }
//     }
//
//     pub fn trayt(&self, path: &ReferencePath) -> Rc<Trait> {
//         self.traits.resolve(path)
//     }
//
//     pub fn class(&self, path: &ReferencePath) -> Rc<Class> {
//         self.classes.resolve(path)
//     }
//
//     pub fn lett(&self, path: &ReferencePath) -> Rc<Let> {
//         self.lets.resolve(path)
//     }
//
//     pub fn def(&self, path: &ReferencePath) -> Rc<Definition> {
//         self.defs.resolve(path)
//     }
//
//     pub fn add_trait(&mut self, path: ReferencePath, trayt: Trait) {
//         self.traits.add(path, trayt)
//     }
//
//     pub fn add_let(&mut self, path: ReferencePath, lett: Let) {
//         self.lets.add(path, lett)
//     }
//
//     pub fn add_module(&mut self, root_path: &ReferencePath, module: Module) {
//         for (name, trayt) in module.traits {
//             self.add_trait(Self::join_path(root_path, name), trayt);
//         }
//
//         for (name, class) in module.classes {
//             self.add_class(Self::join_path(root_path, name), class)
//         }
//
//         for (name, strukt) in module.structs {
//             self.add_struct(Self::join_path(root_path, name), strukt)
//         }
//
//         for (name, def) in module.defs {
//             self.add_def(path(&name), def)
//         }
//
//         // TODO: add lets
//     }
//
//     pub fn add_class(&mut self, path: ReferencePath, class: Class) {
//         self.classes.add(path.clone(), class);
//         self.lets
//             .add(path.clone(), self.classes.resolve(&path).constructor());
//     }
//
//     pub fn add_struct(&mut self, path: ReferencePath, strukt: Struct) {
//         self.structs.add(path.clone(), strukt);
//         self.lets
//             .add(path.clone(), self.structs.resolve(&path).constructor());
//     }
//
//     pub fn add_def(&mut self, path: ReferencePath, def: Definition) {
//         self.defs.add(path, def)
//     }
//
//     fn join_path(root: &ReferencePath, end: String) -> ReferencePath {
//         if end.is_empty() {
//             root.clone()
//         } else {
//             root.iter().cloned().chain(std::iter::once(end)).collect()
//         }
//     }
//
//     pub fn local_scope(
//         &self,
//         zelf: Option<Rc<Instance>>,
//         locals: HashMap<String, Rc<Instance>>,
//     ) -> LocalScope {
//         LocalScope {
//             locals,
//             zelf,
//             scope: self,
//         }
//     }
// }
//
// impl LocalScope<'_> {
//     pub fn local(&self, name: &str) -> &Rc<Instance> {
//         self.locals.get(name).unwrap()
//     }
//
//     pub fn zelf(&self) -> &Option<Rc<Instance>> {
//         &self.zelf
//     }
//
//     pub fn scope(&self) -> &Scope {
//         self.scope
//     }
// }

// TODO: move to separate module. Move logic of path() function into it.
/// A table of references to items of a kind.
pub struct Table<T> {
    items: Vec<(ReferencePath, Rc<T>)>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn resolve(&self, path: &ReferencePath) -> CResult<Rc<T>> {
        let matched_references = self
            .items
            .iter()
            .filter(|reference| reference_matches(&reference.0, path))
            .collect::<Vec<_>>();

        match matched_references.len() {
            0 => error(format!("No resolution for {}", path.join("\\")), 0),
            1 => Ok(Rc::clone(&matched_references.first().unwrap().1)),
            _ => error(format!("Multiple resolutions for {}", path.join("\\")), 0),
        }
    }

    pub fn add(&mut self, path: ReferencePath, item: T) {
        // TODO: change into hashmap. Error on conflict.
        self.items.push((path, Rc::new(item)))
    }
}

pub fn reference_matches(own_path: &ReferencePath, path: &ReferencePath) -> bool {
    let own_path_len = own_path.len();

    if path.len() > own_path_len {
        return false;
    }

    path.iter()
        .rev()
        .enumerate()
        .all(|(i, segment)| segment == own_path.get(own_path_len - i - 1).unwrap())
}
