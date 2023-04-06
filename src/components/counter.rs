pub use crate::components::generated::counter::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Hash, Debug, PartialEq)]
pub struct Metric {
    name: String,
    labels: Vec<Label>,
}

#[derive(Hash, Debug, PartialEq)]
pub struct Label {
    name: String,
    value: String,
}

pub(crate) fn job(input: Inputs, output: OutputPorts) -> JobResult {
    let label1 = Label {
        name: "foo".to_string(),
        value: "1".to_string(),
    };
    let label2 = Label {
        name: "bar".to_string(),
        value: "2".to_string(),
    };

    let metric = Metric {
        name: "total_requests".to_string(),
        labels: vec![label1, label2],
    };

    println!("{:?}", metric);

    Ok(())
}
