use super::{
    activity::{Activity, EstimationValue},
    pert::{Pert, PertId},
};
use cli_table::{format::Justify, Table, WithTitle};

#[derive(Table)]
struct Row {
    #[table(title = "Activity", justify = "Justify::Right")]
    name: String,
    #[table(title = "Optimistic")]
    optimistic: EstimationValue,
    #[table(title = "Probable")]
    probable: EstimationValue,
    #[table(title = "Pessimistic")]
    pessimistic: EstimationValue,
    #[table(title = "PERT estimation")]
    pert: f64,
}

struct PertWithActivities {
    pert: Pert,
    activities: Vec<Activity>,
}
pub struct Report {
    data: PertWithActivities,
}

impl Report {
    pub fn new(pert: Pert, activities: Vec<Activity>) -> Self {
        Self {
            data: PertWithActivities { pert, activities },
        }
    }

    fn estimated_total(&self) -> f64 {
        self.data
            .activities
            .iter()
            .map(|activity| activity.estimated())
            .sum()
    }

    pub fn table(&mut self) -> String {
        let rows: Vec<Row> = self
            .data
            .activities
            .iter()
            .map(|activity| Row {
                name: activity.name.to_owned(),
                pessimistic: activity.estimation.pessimistic,
                probable: activity.estimation.probable,
                optimistic: activity.estimation.optimistic,
                pert: activity.estimated(),
            })
            .collect();
        let rows_str = rows.with_title().display().unwrap().to_string();

        format!(
            "Project: {}\n\n{}\nTOTAL: {}",
            self.data.pert.name,
            rows_str,
            self.estimated_total()
        )
    }
}

#[derive(Table)]
struct ListRowPert {
    #[table(title = "ID", justify = "Justify::Right")]
    id: PertId,
    #[table(title = "Name")]
    name: String,
}

pub fn list_view(perts: Vec<Pert>) -> String {
    let list: Vec<ListRowPert> = perts
        .into_iter()
        .map(|pert| ListRowPert {
            id: pert.id,
            name: pert.name,
        })
        .collect();
    list.with_title().display().unwrap().to_string()
}

#[cfg(test)]
mod test {
    use super::Report;
    use crate::modules::{activity::Activity, pert::Pert};

    #[test]
    fn table() {
        let pert = Pert::new(1, "example".to_string());
        let activities = vec![
            Activity::new("activity 1".to_string(), 6, 10, 15),
            Activity::new("activity 2".to_string(), 18, 25, 39),
            Activity::new("activity 3".to_string(), 14, 22, 35),
            Activity::new("activity 4".to_string(), 23, 34, 62),
        ];

        let mut report = Report::new(pert, activities);
        insta::assert_display_snapshot!(report.table());
    }
}
