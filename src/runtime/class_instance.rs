// use crate::runtime::instance::Instance;
// use crate::sem::class::Class;
// use std::collections::HashMap;
// use std::rc::Rc;
//
// pub struct ClassInstance {
//     class: Rc<Class>,
//     dependencies: HashMap<String, Rc<Instance>>,
// }
//
// impl ClassInstance {
//     pub fn new(class: &Rc<Class>, dependencies: HashMap<String, Rc<Instance>>) -> Self {
//         Self {
//             class: Rc::clone(class),
//             dependencies,
//         }
//     }
//
//     pub fn class(&self) -> &Rc<Class> {
//         &self.class
//     }
//
//     pub fn dependency(&self, name: &str) -> &Rc<Instance> {
//         self.dependencies
//             .get(name)
//             .unwrap_or_else(|| panic!("Dependency {} does not exist", name))
//     }
//
//     pub fn dependencies(&self) -> HashMap<String, Rc<Instance>> {
//         self.dependencies.clone()
//     }
// }
