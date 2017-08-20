extern crate handlebars;
extern crate pulldown_cmark;
extern crate serde_json;
extern crate linked_hash_map;

pub mod parameter;
pub mod context;

use std::io::{Error,ErrorKind,Result};
use render::context::ContextFile;

// TODO: allow other templates
fn template() -> &'static str {
    return include_str!("../config/default.html.hbs");
}

pub fn render(context : ContextFile, content_doc : String) -> Result<String> {
    let mut content = String::new();
    let parser = pulldown_cmark::Parser::new(&content_doc);
    pulldown_cmark::html::push_html(&mut content, parser);

    let mut renderer = handlebars::Handlebars::new();
    renderer.register_template_string("html", template())
        .map_err(|_| invalid_data_err("compiled with invalid template"))?;

    let data = PlaybookData {
        content,
        parameters: serde_json::to_string_pretty(&context.parameters)
            .map_err(|err| invalid_data_err(&format!("unable to map parameters to json: {}", err)))?
    };

    let result = renderer.render("html", &data)
        .map_err(|err| invalid_data_err(format!("{:?}", err).as_str()))?;

    return Ok(result);
}

#[derive(Serialize)]
struct PlaybookData {
    content : String,
    parameters : String
}

fn invalid_data_err(reason : &str) -> Error {
    return Error::new(ErrorKind::InvalidData, reason);
}
