use anyhow::Result;
use perty::{
    modules::{pert::PertId, storage::PostgresDb},
    perty::Perty,
};

fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().expect("Unable to load environment variables");
    println!("Hello, welcome to Perty!");

    let storage = PostgresDb::new()?;
    const PERT_ID: PertId = 1;

    let mut perty = Perty::new(Box::new(storage));
    let pert = perty.get_pert(PERT_ID)?;
    if pert.is_none() {
        println!("Adding new PERT");
        perty.add_pert(PERT_ID, "my first PERT".to_string())?;
    } else {
        println!("PERT already exists");
    }
    // perty
    //     .add_activity(PERT_ID, Activity::new("activity 1".to_string(), 10, 20, 30))?;
    // perty
    //     .add_activity(PERT_ID, Activity::new("activity 2".to_string(), 15, 20, 40))?;

    let mut report = perty.get_reporter(PERT_ID)?;
    println!("{}", report.table());

    Ok(())
}
