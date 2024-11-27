#[cfg(test)]
mod tests {

    use quick_xml::de::from_str;

    use crate::{
        node::{self, NodeRegistration},
        pipeline::{get_any, set_any, Pipeline},
    };
    fn parse_xml(xml: &str) -> anyhow::Result<Pipeline> {
        let pipeline: Pipeline = from_str(xml)?;
        Ok(pipeline)
    }

    struct GetInputNode;

    impl node::Node for GetInputNode {
        fn execute(&self, context: &mut node::AnyMap, _input_id: String, output_id: String) {
            set_any(context, &output_id, "hello".to_string());

            println!("{:?}", context);
        }
    }

    struct PrintInputNode;

    impl node::Node for PrintInputNode {
        fn execute(&self, context: &mut node::AnyMap, input_id: String, _output_id: String) {
            println!("input_id {}", input_id);
            let t = get_any::<String>(context, &input_id);
            println!("output------> {:?}", t);
        }
    }

    // 注册具体节点
    inventory::submit! {
        NodeRegistration {
                class_name: "GetInputNode",
                 constructor: || Box::new(GetInputNode),
             }
    }

    inventory::submit! {
        NodeRegistration {
                class_name: "PrintInputNode",
                 constructor: || Box::new(PrintInputNode),
             }
    }

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let data = include_bytes!(r"test.xml");
        let pipeline = parse_xml(std::str::from_utf8(data)?)?;
        for action in &pipeline.actions {
            println!("{:?}", action);
        }

        println!("execute");

        pipeline.execute(None, Some(|x| println!("aaaaa  {}", x)));

        anyhow::Ok(())
    }
}
