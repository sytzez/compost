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
