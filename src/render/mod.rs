extern crate yaml_rust;
extern crate handlebars;
extern crate aurelius;

pub fn render(parameters : yaml_rust::Yaml, content : String) -> String {
    return aurelius::markdown::to_html_cmark(content.as_str());
}
