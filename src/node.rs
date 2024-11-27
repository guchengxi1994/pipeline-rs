use std::{any::Any, collections::HashMap};

pub type AnyMap = HashMap<String, Box<dyn Any>>;

pub trait Node {
    fn execute(&self, context: &mut AnyMap, input_id: String, output_id: String);
}

pub struct NodeRegistration {
    pub class_name: &'static str,
    pub constructor: fn() -> Box<dyn Node>,
}

inventory::collect!(NodeRegistration);

pub fn get_node(class_name: &str) -> Option<Box<dyn Node>> {
    for registration in inventory::iter::<NodeRegistration> {
        if registration.class_name == class_name {
            return Some((registration.constructor)());
        }
    }
    None
}
