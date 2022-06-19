use anyhow::Result;
use std::fs::File;
use std::io;
use std::io::Write;

use crate::{
    modules::{activity::Estimation, pert::PertId, view::list_view},
    perty::Perty,
};

pub enum Output {
    Console,
    HTML,
    CSV,
}

fn create_file(file_name: String, content: String) -> Result<String> {
    let file_path = std::env::current_dir()?.join(file_name);
    let mut temp_file = File::create(&file_path)?;
    writeln!(temp_file, "{}", content)?;
    Ok(file_path.to_str().unwrap().to_string())
}

fn file_in_browser(pert_id: PertId, content: String) -> Result<()> {
    let file_name = format!("report-{}.html", pert_id);
    let file_path = create_file(file_name, content)?;
    let file_path_url = format!("file://{}", file_path);
    webbrowser::open(&file_path_url).expect("Unable to open browser");
    println!("Opened browser...");

    Ok(())
}

pub fn read_input() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

pub fn create_pert(mut perty: Perty) -> Result<()> {
    println!("Name: ");
    let pert_name = read_input()?;
    println!("Creating PERT {}...", pert_name);
    let pert_id = perty.add_pert(pert_name).unwrap();
    println!("Created {}", pert_id);
    Ok(())
}

pub fn list_perts(mut perty: Perty) -> Result<()> {
    println!("Getting list of PERTs...");
    let perts = perty.get_perts()?;
    println!("{}", list_view(perts));
    Ok(())
}

pub fn get_pert(mut perty: Perty, pert_id: PertId, output: Output) -> Result<()> {
    println!("Getting list of PERTs...");
    if let Some(report) = perty.get_reporter(pert_id)? {
        match output {
            Output::Console => println!("{}", report.pert_detail().ascii()),
            Output::HTML => {
                file_in_browser(pert_id, report.pert_detail().html())?;
            }
            Output::CSV => {
                let file_name = format!("report-{}.csv", pert_id);
                create_file(file_name, report.pert_detail().csv())?;
            }
        };
    } else {
        println!("No PERT found with id {}", pert_id);
    }
    Ok(())
}

pub fn add_dependency(mut perty: Perty, pert_id: PertId) -> Result<()> {
    let reporter = perty.get_reporter(pert_id)?;
    if reporter.is_none() {
        println!("No PERT found with id {}", pert_id);
        return Ok(());
    }
    println!("{}", reporter.unwrap().list_activities());
    println!("\"A\" depends on \"B\"");
    println!("Insert the ID of activity \"A\":");
    let activities = perty.get_activities(pert_id)?;

    let tail_id: PertId = read_input()?.parse()?;
    let tail = activities
        .iter()
        .find(|act| (*act).id == tail_id)
        .expect("Unable to find activity A");
    let tail_name = &tail.name;
    println!("Insert the ID of activity \"B\":");
    let head_id: PertId = read_input()?.parse()?;
    let head = activities
        .iter()
        .find(|act| act.id == head_id)
        .expect("Unable to find activity B");
    let head_name = &head.name;

    println!(
        "You are adding the following dependencies: \"{}\" depends on \"{}\"",
        tail_name, head_name
    );
    println!("Are you sure? Y/N");
    match read_input()?.as_str() {
        "Y" => {
            perty.add_dependency(head_id, tail_id)?;
            println!("Dependency added");
        }
        "N" => {
            println!("The dependency has NOT been added.");
        }
        _ => {
            panic!("Unknown input, let's do nothing... Retry");
        }
    }
    Ok(())
}

pub fn add_activity(mut perty: Perty, pert_id: PertId) -> Result<()> {
    println!("Add estimated cost:");
    println!("Activity: ");
    let activity_name = read_input()?;
    println!("Optimistic: ");
    let optimistic = read_input()?.parse()?;
    println!("Most probable: ");
    let probable = read_input()?.parse()?;
    println!("Pessimistic: ");
    let pessimistic = read_input()?.parse()?;
    perty.add_activity(
        pert_id,
        activity_name,
        Estimation {
            optimistic,
            probable,
            pessimistic,
        },
    )?;
    Ok(())
}
