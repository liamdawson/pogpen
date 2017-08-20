#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate pulldown_cmark;
extern crate serde_yaml;

use clap::{ArgMatches};
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
        Some("render") => {
            match render_from_args(matches.subcommand_matches("render").unwrap()) {
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(2);
                },
                _ => println!("playbook successfully rendered")
            }
        }
        Some(_) => unreachable!(),
        None => unreachable!()
    }
}

fn render_from_args(matches : &ArgMatches) -> std::io::Result<()> {
    let context = load_context(matches.value_of("PARAMETERS").unwrap())?;
    let content = load_content(matches.value_of("CONTENT").unwrap())?;

    let mut out_file = std::fs::File::create(matches.value_of("OUTPUT").unwrap())?;

    out_file.write_all(render::render(context, content)?.as_bytes())?;
    out_file.flush()?;

    return Ok(());
}

fn load_context(path : &str) -> std::io::Result<render::context::ContextFile> {
    let mut context_str = String::new();
    let mut context_file = std::fs::File::open(path)?;
    context_file.read_to_string(&mut context_str)?;

    let context : render::context::ContextFile = serde_yaml::from_str(&mut context_str)
        .map_err(|err| invalid_data_err(&format!("unable to read context file: {}", err)))?;

    return Ok(context);
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