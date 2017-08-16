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
    param_field_type: String
} 

fn map_parameters(doc : &yaml_rust::Yaml) -> Result<Vec<PlaybookParameter>> {
    let mut out_params = Vec::new();

    let parameters_section = &doc["parameters"];

    if parameters_section.is_badvalue() {
        return Err(invalid_data_err("parameters file does not contain a 'parameters' key"));
    }

    if let yaml_rust::Yaml::Hash(ref params) = *parameters_section {
        for (key, val) in params {
            match map_parameter(key, val) {
                Ok(param) => {
                    out_params.push(param);
                },
                Err(error) => {
                    println!("unable to process parameter {:?}: {:?}", key, error);
                }
            }
        }
    }

    return Ok(out_params);
}

fn map_parameter(key : &yaml_rust::Yaml, val : &yaml_rust::Yaml) -> Result<PlaybookParameter> {
    let id = key.as_str().unwrap_or("");

    if id == "" {
        return Err(invalid_data_err(format!("could not determine the ID for this parameter: {:?}", key).as_str()));
    }

    let param_type = val["type"].as_str().unwrap_or("string").to_string();
    let param_field_type = String::from(if param_type.as_str() == "password" {
            "password"
    } else {
        "text"
    });

    return Ok(PlaybookParameter {
        name: val["name"].as_str().unwrap_or(id).to_string(),
        id: id.to_string(),
        value: val["value"].as_str().unwrap_or("").to_string(),
        param_type,
        param_field_type: param_field_type.to_string()
    });
}

fn invalid_data_err(reason : &str) -> Error {
    return Error::new(ErrorKind::InvalidData, reason);
}
