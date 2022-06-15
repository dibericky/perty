use anyhow::Result;
use dotenv::dotenv;
use perty::modules::activity::Activity;
use perty::modules::view::list_view;
use perty::{
    modules::{pert::PertId, storage::PostgresDb},
    perty::Perty,
};
use std::env;
use std::io;

fn read_input() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn create_pert(mut perty: Perty) -> Result<()> {
    println!("Name: ");
    let pert_name = read_input()?;
    println!("Creating PERT {}...", pert_name);
    let pert_id = perty.add_pert(pert_name).unwrap();
    println!("Created {}", pert_id);
    Ok(())
}

fn list_perts(mut perty: Perty) -> Result<()> {
    println!("Getting list of PERTs...");
    let perts = perty.get_perts()?;
    println!("{}", list_view(perts));
    Ok(())
}

fn get_pert(mut perty: Perty, pert_id: PertId) -> Result<()> {
    println!("Getting list of PERTs...");
    if let Some(mut report) = perty.get_reporter(pert_id)? {
        println!("{}", report.table());
    } else {
        println!("No PERT found with id {}", pert_id);
    }
    Ok(())
}

fn add_activity(mut perty: Perty, pert_id: PertId) -> Result<()> {
    println!("Add estimated cost:");
    println!("Activity: ");
    let activity_name = read_input()?;
    println!("Optimistic: ");
    let optimistic = read_input()?.parse()?;
    println!("Most probable: ");
    let probable = read_input()?.parse()?;
    println!("Pessimistic: ");
    let pessimistic = read_input()?.parse()?;
    let activity = Activity::new(activity_name, optimistic, probable, pessimistic);
    perty.add_activity(pert_id, activity)?;
    Ok(())
}

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
                create_pert(perty)?;
            }
            x if *x == "list" => {
                list_perts(perty)?;
            }
            _ => panic!("unknown command {}", a),
        },
        [a, b] => match a {
            x if *x == "get" => {
                get_pert(perty, b.parse()?)?;
            }
            _ => panic!("unknown command {} {}", a, b),
        },
        [a, b, c, d] => match a {
            x if *x == "edit" => {
                let operation = vec![c.to_string(), d.to_string()];
                match operation.join(" ") {
                    x if x == "add activity" => {
                        add_activity(perty, b.parse()?)?;
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
