use perty::modules::{activity::Activity, pert::Pert, view::Report};

fn main() {
    println!("Hello, welcome to Perty!");

    // perty create
    let mut my_pert = Pert::new("my first PERT".to_string());

    // perty edit 1 add activity
    my_pert
        .add_activity(Activity::new("activity 1".to_string(), 10, 20, 30))
        .add_activity(Activity::new("activity 2".to_string(), 15, 20, 40));

    let report = Report::new(my_pert);
    println!("{}", report.table());
}
