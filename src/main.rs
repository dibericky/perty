use perty::modules::{activity::Activity, pert::Pert, storage::MemoryStorage, view::Report};

fn main() {
    println!("Hello, welcome to Perty!");
    let storage = MemoryStorage::new();
    // perty create
    let mut my_pert = Pert::new("my first PERT".to_string(), Box::new(storage));

    // perty edit 1 add activity
    my_pert
        .add_activity(Activity::new("activity 1".to_string(), 10, 20, 30))
        .add_activity(Activity::new("activity 2".to_string(), 15, 20, 40));

    let mut report = Report::new(my_pert);
    println!("{}", report.table());
}
