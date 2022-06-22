use anyhow::Result;
use dotenv::dotenv;
use perty::{
    modules::{pert::PertId, storage::PostgresDb},
    perty::Perty,
    perty_cli::{self, Output},
};
use std::env::{self, Args};

fn assert_no_rest(args: &mut Args) {
    let other = args.next();
    if other.is_some() {
        let args_arr: Vec<String> = args.collect();
        panic!("unknown command {}", args_arr.join(" "));
    }
}

fn main() -> Result<(), anyhow::Error> {
    dotenv().expect("Unable to load environment variables");
    println!("Hello, welcome to Perty!");

    let storage = PostgresDb::new()?;
    let mut perty = Perty::new(Box::new(storage));

    let mut args = env::args();
    args.next();

    let operation = args.next().expect("invalid arguments");

    match operation.as_str() {
        "create" => {
            let resource = args.next();
            if resource.is_none() {
                return perty_cli::create_pert(perty);
            }
            let resource = resource.unwrap();
            match resource.as_str() {
                "board" => {
                    let platform = args.next().expect("Missing platform argument");
                    match platform.as_str() {
                        "--github" => {
                            perty_cli::create_board_github(perty)?;
                        }
                        _ => panic!("Unsupported argument"),
                    }
                }
                _ => panic!("Invalid command"),
            }
        }
        "list" => {
            assert_no_rest(&mut args);
            perty_cli::list_perts(&mut perty)?;
        }
        "get" => {
            let resource = args.next().expect("missing resource to create in command");
            let pert_id: PertId = resource.parse()?;
            match args.next().expect("unknown command").as_str() {
                "pert" => {
                    let mut output = Output::Console;
                    if let Some(format) = args.next() {
                        output = match format.as_str() {
                            "--html" => Output::HTML,
                            "--csv" => Output::CSV,
                            _ => panic!("Unknown format {}", format),
                        }
                    }
                    perty_cli::get_pert(perty, pert_id, output)?;
                }
                "roadmap" => {
                    let mut output = Output::Console;
                    if let Some(format) = args.next() {
                        output = match format.as_str() {
                            "--html" => Output::HTML,
                            "--csv" => Output::CSV,
                            _ => panic!("Unknown format {}", format),
                        }
                    }
                    perty_cli::get_roadmap(perty, pert_id, output)?;
                }
                _ => panic!("Unknown command"),
            }
        }
        "edit" => {
            let resource = args.next();
            let resource = resource.expect("missing resource to create in command");
            let pert_id: PertId = resource.parse()?;
            let operation = args
                .next()
                .expect("missing operation to perform in command");

            match operation.as_str() {
                "add" => {
                    let what_add = args.next().expect("Missing resource to add in command");
                    match what_add.as_str() {
                        "activity" => {
                            perty_cli::add_activity(perty, pert_id)?;
                        }
                        "dependency" => {
                            perty_cli::add_dependency(perty, pert_id)?;
                        }
                        _ => panic!("Unknown resource to add {}", what_add),
                    }
                }
                _ => panic!("unknown operation {}", operation),
            }
        }
        _ => panic!("invalid operation {}", operation),
    }
    Ok(())
}
