extern crate yaml_rust;
extern crate handlebars;
extern crate aurelius;

use std::io::{Error,ErrorKind};
use std::collections::BTreeMap;

// TODO: allow other templates
fn template() -> &'static str {
    return include_str!("../config/default.html.hbs");
}

pub fn render(parameters_doc : yaml_rust::Yaml, content : String) -> Result<String, Error> {
    let content = aurelius::markdown::to_html_cmark(content.as_str());

    let mut renderer = handlebars::Handlebars::new();
    renderer.register_template_string("html", template())
        .map_err(|_| invalid_data_err("compiled with invalid template"))?;

    let mut data : BTreeMap<String, String> = BTreeMap::new();
    data.insert("playbookContent".to_string(), content);

    let result = renderer.render("html", &data)
        .map_err(|err| invalid_data_err(format!("{:?}", err).as_str()))?;

    return Ok(result);
}

fn invalid_data_err(reason : &str) -> Error {
    return Error::new(ErrorKind::InvalidData, reason);
}
