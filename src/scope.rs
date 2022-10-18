use std::rc::Rc;
use crate::class::{Class, Let, Struct, Trait};
use crate::module::Module;

pub type ReferencePath = Vec<String>;

pub fn create_reference_path(string: &str) -> ReferencePath {
    string.split(".")
        .map(|segment| segment.to_string())
        .collect()
}

pub struct Scope {
    traits: References<Trait>,
    classes: References<Class>,
    structs: References<Struct>,
    lets: References<Let>,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            traits: References::new(),
            classes: References::new(),
            structs: References::new(),
            lets: References::new(),
        }
    }

    pub fn trayt(&self, path: &str) -> Rc<Trait> {
        self.traits.resolve(path)
    }

    pub fn class(&self, path: &str) -> Rc<Class> {
        self.classes.resolve(path)
    }

    pub fn lett(&self, path: &str) -> Rc<Let> {
        self.lets.resolve(path)
    }

    pub fn add_trait(&mut self, path: &str, trayt: Trait) {
        self.traits.add(path, trayt)
    }

    pub fn add_module(&mut self, path: &str, module: Module) {
        for (name, trayt) in module.traits {
            self.add_trait(&Self::join_path(path, &name), trayt);
        }

        for (name, class) in module.classes {
            self.add_class(&Self::join_path(path, &name), class)
        }

        for (name, strukt) in module.structs {
            self.add_struct(&Self::join_path(path, &name), strukt)
        }
    }

    pub fn add_class(&mut self, _path: &str, _class: Class) {
        // TODO: add class
        // TODO: add class constructor fn
    }

    pub fn add_struct(&mut self, path: &str, strukt: Struct) {
        self.structs.add(path, strukt);
        self.lets.add(path, self.structs.resolve(path).constructor());
    }

    fn join_path<'a>(root: &str, relative: &str) -> String {
        if relative.is_empty() {
            root.to_owned()
        } else {
            format!("{}.{}", root, relative)
        }
    }

    pub fn create_sub_scope(&self) -> Self {
        panic!()
    }
}

pub struct References<T> {
    references: Vec<(ReferencePath, Rc<T>)>,
}

impl<T> References<T> {
    pub fn new() -> Self {
        Self {
            references: Vec::new(),
        }
    }

    pub fn resolve(&self, path: &str) -> Rc<T> {
        let path = &create_reference_path(path);

        let matched_references = self
            .references
            .iter()
            .filter(|reference| reference_matches(&reference.0, path))
            .collect::<Vec<_>>();

        match matched_references.len() {
            0 => panic!(),
            1 => Rc::clone(&matched_references.first().unwrap().1),
            _ => panic!(),
        }
    }

    pub fn add(&mut self, path: &str, item: T) {
        self.references.push((create_reference_path(path), Rc::new(item)))
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
