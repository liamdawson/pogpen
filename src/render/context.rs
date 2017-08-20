use render::linked_hash_map::LinkedHashMap;
use render::parameter::ParameterDetail;

#[derive(Serialize, Deserialize, Debug)]
pub struct ContextFile {
    pub parameters : LinkedHashMap<String, ParameterDetail>
}