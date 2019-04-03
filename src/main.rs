#[macro_use]               //use the lazy_static! macro
extern crate lazy_static;  //from the lazy_static crate

use std::string::String;
use std::vec::Vec;
use std::io::stdin;
use std::process::Command;

/**
 * @brief Outputs:
 *     static vector containing rust binary
 *     build types.
 */
lazy_static! {
    static ref OUTPUTS: Vec<&'static str> = {
        let mut outputs = Vec::new();
        outputs.push("bin");
        outputs.push("lib");
        outputs
    };
}

/**
 * @brief main:
 *     main program function
 */
fn main() {
    let mut name   = String::new(); //name of new project
    let mut output = String::new(); //type of new project

    //get and sanitize project name
    println!("project name:");
    stdin().read_line(&mut name)
           .expect("");
    name = name.trim_end()
               .to_string();

    //get and sanitize project type
    println!("project type: ");
    stdin().read_line(&mut output)
           .expect("");
    output = output.trim_end()
                   .to_string()
                   .to_lowercase();
    if !(OUTPUTS.contains(&&output.as_ref())) {
        println!("{0} is not a valid project type", output);
        return;
    }
    output = format!("--{0}", output);

    //build and run command
    println!("running: cargo new {0} {1}", name, output);
    Command::new("cargo")
            .arg("new")
            .arg(name)
            .arg(output)
            .output()
            .expect("failed to run cargo");
}