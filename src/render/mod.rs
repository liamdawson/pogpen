extern crate yaml_rust;
extern crate handlebars;
extern crate aurelius;

use std::io::{Error,ErrorKind,Result};

// TODO: allow other templates
fn template() -> &'static str {
    return include_str!("../config/default.html.hbs");
}

pub fn render(parameters_doc : &yaml_rust::Yaml, content : String) -> Result<String> {
    let content = aurelius::markdown::to_html_cmark(content.as_str());

    let mut renderer = handlebars::Handlebars::new();
    renderer.register_template_string("html", template())
        .map_err(|_| invalid_data_err("compiled with invalid template"))?;

    let mut playbook_parameters : Vec<PlaybookParameter> = Vec::new();
    playbook_parameters.push(map_parameter());

    let data = PlaybookData {
        content,
        parameters: map_parameters(&parameters_doc)?
    };

    let result = renderer.render("html", &data)
        .map_err(|err| invalid_data_err(format!("{:?}", err).as_str()))?;

    return Ok(result);
}

#[derive(Serialize)]
struct PlaybookData {
    content : String,
    parameters : Vec<PlaybookParameter>
}

#[derive(Serialize)]
struct PlaybookParameter {
    id: String,
    name: String,
    param_type: String,
    value: String,
    secure: bool
} 

fn map_parameters(doc : &yaml_rust::Yaml) -> Result<Vec<PlaybookParameter>> {
    let mut out_params = Vec::new();

    let parameters_section = &doc["parameters"];

    if parameters_section.is_badvalue() {
        return Err(invalid_data_err("parameters file does not contain a 'parameters' key"));
    }

    if let yaml_rust::Yaml::Hash(ref params) = *parameters_section {
        for (key, val) in params {
            let id = key.as_str().unwrap_or("");

            if id == "" {
                println!("could not determine the ID for this parameter: {:?}", key);
                continue;
            }

            out_params.push(PlaybookParameter {
                name: val["name"].as_str().unwrap_or(id).to_string(),
                id: id.to_string(),
                value: val["value"].as_str().unwrap_or("").to_string(),
                param_type: val["type"].as_str().unwrap_or("string").to_string(),
                secure: val["secure"].as_bool().unwrap_or(false)
            });
        }
    }

    return Ok(out_params);
}

fn map_parameter() -> PlaybookParameter {
    return PlaybookParameter {
        id: "wat".to_string(),
        name: "File Name".to_string(),
        param_type: "string".to_string(),
        value: "params.yml".to_string(),
        secure: false
    };
}

fn invalid_data_err(reason : &str) -> Error {
    return Error::new(ErrorKind::InvalidData, reason);
}
