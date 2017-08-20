use std::collections::HashMap;
use render::parameter::ParameterDetail;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextFile {
    pub parameters : HashMap<String, ParameterDetail>
}