use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextFile {
    parameters: HashMap<String, ParameterDetail>
}

#[derive(Serialize, Deserialize, Debug)]
struct ParameterDetail {
    name: Option<String>,
    format: Option<ParameterFormat>,
    value: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
enum ParameterFormat {
    String,
    Boolean,
    Secret
}
