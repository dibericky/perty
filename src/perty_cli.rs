use anyhow::Result;
use std::io;

use crate::{
    modules::{activity::Activity, pert::PertId, view::list_view},
    perty::Perty,
};

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

pub fn get_pert(mut perty: Perty, pert_id: PertId) -> Result<()> {
    println!("Getting list of PERTs...");
    if let Some(mut report) = perty.get_reporter(pert_id)? {
        println!("{}", report.table());
    } else {
        println!("No PERT found with id {}", pert_id);
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
    let activity = Activity::new(activity_name, optimistic, probable, pessimistic);
    perty.add_activity(pert_id, activity)?;
    Ok(())
}
