#[macro_use]
extern crate colour;
extern crate sidetree_client;
use clap::{load_yaml, App};
use sidetree_client::operations::*;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("create") {
        let create_operation = create().unwrap();

        yellow_ln!("{}", serde_json::to_string_pretty(&create_operation).unwrap());
    }

    if let Some(_) = matches.subcommand_matches("update") {
        red_ln!("'update' subcommand not yet implemented");
    }
}
