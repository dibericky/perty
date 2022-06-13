use super::pert::Pert;
use cli_table::{format::Justify, Table, WithTitle};

#[derive(Table)]
struct Row {
    #[table(title = "Activity", justify = "Justify::Right")]
    name: String,
    #[table(title = "Optimistic")]
    optimistic: u32,
    #[table(title = "Probable")]
    probable: u32,
    #[table(title = "Pessimistic")]
    pessimistic: u32,
    #[table(title = "PERT estimation")]
    pert: f64,
}

pub struct Report {
    pert: Pert,
}

impl Report {
    pub fn new(pert: Pert) -> Self {
        Self { pert }
    }

    pub fn table(&mut self) -> String {
        let rows: Vec<Row> = self
            .pert
            .get_activities()
            .unwrap()
            .into_iter()
            .map(|activity| Row {
                name: activity.name.to_owned(),
                pessimistic: activity.estimation.pessimistic,
                probable: activity.estimation.probable,
                optimistic: activity.estimation.optimistic,
                pert: activity.estimated(),
            })
            .collect();
        let rows_str = rows.with_title().display().unwrap().to_string();

        format!("{}\nTOTAL: {}", rows_str, self.pert.estimated_total())
    }
}

#[cfg(test)]
mod test {
    use super::Report;
    use crate::modules::{activity::Activity, pert::Pert, storage::MemoryStorage};

    #[test]
    fn table() {
        let storage = MemoryStorage::new();
        let mut pert = Pert::new("example".to_string(), Box::new(storage));
        pert.add_activity(Activity::new("activity 1".to_string(), 6, 10, 15))
            .add_activity(Activity::new("activity 2".to_string(), 18, 25, 39))
            .add_activity(Activity::new("activity 3".to_string(), 14, 22, 35))
            .add_activity(Activity::new("activity 4".to_string(), 23, 34, 62));

        let mut report = Report { pert };
        insta::assert_display_snapshot!(report.table());
    }
}
