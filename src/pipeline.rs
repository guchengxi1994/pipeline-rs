use serde::Deserialize;
use std::{
    any::Any,
    fmt::Display,
    panic::{self, AssertUnwindSafe},
};

use crate::node::{get_node, AnyMap};

#[derive(Debug, Deserialize)]
#[serde(rename = "pipeline")]
pub struct Pipeline {
    #[serde(rename = "action")]
    pub actions: Vec<BaseAction>,
}

impl Pipeline {
    pub fn execute(&self, on_error: Option<fn(&str)>, on_step: Option<fn(&str)>) {
        let mut context: AnyMap = std::collections::HashMap::new();

        for action in &self.actions {
            // action.execute(&mut global,action);

            let result = panic::catch_unwind(AssertUnwindSafe(|| {
                node_execute(action.clone(), &mut context)
            }));
            match result {
                Ok(_) => {
                    if let Some(step_fn) = on_step {
                        step_fn(&format!("action {} executed", action.name));
                    }
                }
                Err(err) => {
                    let err_msg;
                    if let Some(s) = err.downcast_ref::<&str>() {
                        err_msg = s.to_string();
                    } else if let Some(s) = err.downcast_ref::<String>() {
                        err_msg = s.clone();
                    } else {
                        err_msg = "Unknown error".to_string();
                    }

                    if let Some(err_fn) = on_error {
                        err_fn(&err_msg);
                    }
                    break;
                }
            }
        }
    }

    pub fn execute_with_input(
        &self,
        context: &mut AnyMap,
        on_error: Option<fn(&str)>,
        on_step: Option<fn(&str)>,
    ) {
        for action in &self.actions {
            // action.execute(&mut global,action);

            let result =
                panic::catch_unwind(AssertUnwindSafe(|| node_execute(action.clone(), context)));
            match result {
                Ok(_) => {
                    if let Some(step_fn) = on_step {
                        step_fn(&format!("action {} executed", action.name));
                    }
                }
                Err(err) => {
                    let err_msg;
                    if let Some(s) = err.downcast_ref::<&str>() {
                        err_msg = s.to_string();
                    } else if let Some(s) = err.downcast_ref::<String>() {
                        err_msg = s.clone();
                    } else {
                        err_msg = "Unknown error".to_string();
                    }

                    if let Some(err_fn) = on_error {
                        err_fn(&err_msg);
                    }
                    break;
                }
            }
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct BaseAction {
    #[serde(rename = "@class")]
    pub class_name: String,
    #[serde(rename = "@inputId")]
    pub input_id: Option<String>,
    #[serde(rename = "@outputId")]
    pub output_id: Option<String>,
    #[serde(rename = "@name")]
    pub name: String,
}

impl Display for BaseAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn node_execute(action: BaseAction, context: &mut AnyMap) {
    let node = get_node(&action.class_name);
    if let Some(node) = node {
        node.execute(
            context,
            action.input_id.unwrap_or("".to_string()),
            action.output_id.unwrap_or("".to_string()),
        );
    } else {
        panic!("node {} not found", action.class_name);
    }
}

// 函数从 HashMap 中获取并转换值
pub fn get_any<'a, T: 'static>(map: &'a AnyMap, key: &'a str) -> Option<&'a T> {
    map.get(key).and_then(|value| value.downcast_ref::<T>()) // 尝试将引用转换为目标类型
}

pub fn set_any(map: &mut AnyMap, key: &str, value: impl Any) {
    map.insert(key.to_string(), Box::new(value));
}

fn handle_any<T: 'static>(value: Box<dyn std::any::Any>) -> Option<T> {
    // 检查是否可以转换为目标类型 T
    if value.is::<T>() {
        // 如果可以，则 downcast 成目标类型 T
        match value.downcast::<T>() {
            Ok(v) => Some(*v), // 解引用 Box 并返回具体类型
            Err(_) => None,    // 理论上不会进入这个分支
        }
    } else {
        None // 类型不匹配时返回 None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestStruct {
        name: String,
        age: i32,
    }

    #[test]
    fn struct_to_struct() {
        let a = Box::new(TestStruct {
            name: "test".to_string(),
            age: 18,
        });

        let b = handle_any::<TestStruct>(a);
        if let Some(b) = b {
            assert_eq!(b.name, "test");
            assert_eq!(b.age, 18);
        } else {
            panic!("not ok");
        }
    }
}
