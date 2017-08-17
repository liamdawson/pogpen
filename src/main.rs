#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;
extern crate yaml_rust;
extern crate pulldown_cmark;

use clap::{App,ArgMatches};
use std::io::{Read,Write};

mod render;

fn main() {
    let app = clap_app!(app =>
        (name: crate_name!())
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@setting SubcommandRequiredElseHelp)
        (@subcommand render => 
            (about: "renders input to a playbook file")
            (@arg PARAMETERS: * "parameters file (as YAML)")
            (@arg CONTENT: * "content file (as Markdown)")
            (@arg OUTPUT: * "destination file for rendered playbook (as HTML)")
        ));

    let matches = app.get_matches();

    match matches.subcommand_name() {
        Some("render") => render_from_args(matches.subcommand_matches("render").unwrap()).unwrap(),
        Some(_) => unreachable!(),
        None => unreachable!()
    }

    println!("finished");
}

fn render_from_args(matches : &ArgMatches) -> std::io::Result<()> {
    let params = load_params(matches.value_of("PARAMETERS").unwrap())?;
    let content = load_content(matches.value_of("CONTENT").unwrap())?;

    let result = render::render(&params, content);

    let mut out_file = std::fs::File::create(matches.value_of("OUTPUT").unwrap())?;
    out_file.write_all(result?.as_bytes())?;
    out_file.flush()?;

    return Ok(());
}

fn load_params(path : &str) -> std::io::Result<yaml_rust::Yaml> {
    let mut params_str = String::new();
    let mut params_file = std::fs::File::open(path)?;
    params_file.read_to_string(&mut params_str)?;

    let mut params_yaml = yaml_rust::YamlLoader::load_from_str(&mut params_str)
        .map_err(|err| { invalid_data_err(&format!("unable to read YAML data: {:?}", err)) })?;

    return match params_yaml.pop() {
        Some(yaml) => Ok(yaml),
        None => Err(invalid_data_err("unable to find a valid YAML document in the parameters input file"))
    }
}

fn load_content(path : &str) -> std::io::Result<String> {
    let mut contents_str = String::new();
    let mut contents_file = std::fs::File::open(path)?;
    contents_file.read_to_string(&mut contents_str)?;

    return Ok(contents_str);
}

fn invalid_data_err(reason : &str) -> std::io::Error {
    return std::io::Error::new(std::io::ErrorKind::InvalidData,
        reason);
}