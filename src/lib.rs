use pipeline::Pipeline;
use quick_xml::de::from_str;

pub mod node;
pub mod pipeline;
pub mod pipeline_item;
mod test;

pub fn parse_xml(xml: &str) -> anyhow::Result<Pipeline> {
    let pipeline: Pipeline = from_str(xml)?;
    Ok(pipeline)
}
