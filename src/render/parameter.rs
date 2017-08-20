#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "format")]
pub enum ParameterDetail {
    Text { name : Option<String>, value : Option<String> },
    Secret { name : Option<String>, value : Option<String> },
    Boolean { name : Option<String>, value : Option<bool> },
    Choice { name : Option<String>, values : Vec<String> },
    Number { name : Option<String>, value : Option<i32> }
}
