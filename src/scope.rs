use std::collections::HashMap;
use std::rc::Rc;
use crate::class::Class;
use crate::instance::Instance;
use crate::lett::Let;
use crate::module::Module;
use crate::strukt::Struct;
use crate::trayt::Trait;

pub type ReferencePath = Vec<String>;

pub fn path(string: &str) -> ReferencePath {
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

pub struct LocalScope<'a> {
    locals: HashMap<String, Rc<Instance>>,
    zelf: Option<Rc<Instance>>,
    scope: &'a Scope,
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

    pub fn trayt(&self, path: &ReferencePath) -> Rc<Trait> {
        self.traits.resolve(path)
    }

    pub fn class(&self, path: &ReferencePath) -> Rc<Class> {
        self.classes.resolve(path)
    }

    pub fn lett(&self, path: &ReferencePath) -> Rc<Let> {
        self.lets.resolve(path)
    }
    
    pub fn add_trait(&mut self, path: ReferencePath, trayt: Trait) {
        self.traits.add(path, trayt)
    }

    pub fn add_let(&mut self, path: ReferencePath, lett: Let) {
        self.lets.add(path, lett)
    }

    pub fn add_module(&mut self, path: &ReferencePath, module: Module) {
        for (name, trayt) in module.traits {
            self.add_trait(Self::join_path(path, name), trayt);
        }

        for (name, class) in module.classes {
            self.add_class(Self::join_path(path, name), class)
        }

        for (name, strukt) in module.structs {
            self.add_struct(Self::join_path(path, name), strukt)
        }

        // TODO: add lets
    }

    pub fn add_class(&mut self, _path: ReferencePath, _class: Class) {
        // TODO: add class
        // TODO: add class constructor fn
    }

    pub fn add_struct(&mut self, path: ReferencePath, strukt: Struct) {
        self.structs.add(path.clone(), strukt);
        self.lets.add(path.clone(), self.structs.resolve(&path).constructor());
    }

    fn join_path<'a>(root: &ReferencePath, end: String) -> ReferencePath {
        if end.is_empty() {
            root.clone()
        } else {
            root.iter().cloned().chain(std::iter::once(end)).collect()
        }
    }

    pub fn local_scope(&self, zelf: Option<Rc<Instance>>, locals: HashMap<String, Rc<Instance>>) -> LocalScope {
        LocalScope {
            locals,
            zelf,
            scope: self,
        }
    }
}

impl LocalScope<'_> {
    pub fn local(&self, name: &str) -> &Rc<Instance> {
        self.locals.get(name).unwrap()
    }

    pub fn zelf(&self) -> &Option<Rc<Instance>> {
        &self.zelf
    }

    pub fn scope(&self) -> &Scope {
        self.scope
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

    pub fn resolve(&self, path: &ReferencePath) -> Rc<T> {
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

    pub fn add(&mut self, path: ReferencePath, item: T) {
        // TODO: change into hashmap. Error on conflict.
        self.references.push((path, Rc::new(item)))
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
