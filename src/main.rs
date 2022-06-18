use anyhow::Result;
use dotenv::dotenv;
use perty::{
    modules::storage::PostgresDb,
    perty::Perty,
    perty_cli::{self, Output},
};
use std::env;

fn main() -> Result<(), anyhow::Error> {
    dotenv().expect("Unable to load environment variables");
    println!("Hello, welcome to Perty!");

    let storage = PostgresDb::new()?;
    let perty = Perty::new(Box::new(storage));

    let mut args = env::args();
    args.next();
    let commands: Vec<String> = args.collect();
    match &commands[..] {
        [a] => match a {
            x if *x == "create" => {
                perty_cli::create_pert(perty)?;
            }
            x if *x == "list" => {
                perty_cli::list_perts(perty)?;
            }
            _ => panic!("unknown command {}", a),
        },
        [a, b] => match a {
            x if *x == "get" => {
                perty_cli::get_pert(perty, b.parse()?, Output::Console)?;
            }
            _ => panic!("unknown command {} {}", a, b),
        },
        [a, b, c] => match a {
            x if *x == "get" && (*c == "--html" || c == "--csv") => {
                perty_cli::get_pert(
                    perty,
                    b.parse()?,
                    match c {
                        x if x == "--html" => Output::HTML,
                        x if x == "--csv" => Output::CSV,
                        _ => panic!("unknown format {}", c),
                    },
                )?;
            }
            x if *x == "edit" && *c == "dependency" => {
                perty_cli::add_dependency(perty, b.parse()?)?;
            }
            _ => panic!("unknown command {} {} {}", a, b, c),
        },
        [a, b, c, d] => match a {
            x if *x == "edit" => {
                let operation = vec![c.to_string(), d.to_string()];
                match operation.join(" ") {
                    x if x == "add activity" => {
                        perty_cli::add_activity(perty, b.parse()?)?;
                    }
                    _ => panic!("unknown command {} {} {} {}", a, b, c, d),
                }
            }
            _ => panic!("unknown command {} {} {} {}", a, b, c, d),
        },
        _ => panic!("invalid arguments {:?}", commands),
    };

    Ok(())
}
