use std::env;
use switch_statement::switch;
mod setups;

fn main() {
    let args: Vec<String> = env::args().collect();

    let _language = &args[1];
    let workspace_name = &args[2];
    match workspace_name {
        _ => return,
    }
    switch! { _language;
        "--help" => println!("Current languages are: JS"),
        "js" => setups::setup_js(workspace_name.to_string()),
        _ => println!("No input.")
    }
}
