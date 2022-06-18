use super::{
    activity::{Activity, ActivityId, EstimationValue},
    pert::{Pert, PertId},
    pert_report::PertReport,
};
use cli_table::{format::Justify, Table, WithTitle};
use serde::Serialize;

#[derive(Table, Serialize)]
struct ActivityPertRow {
    #[table(title = "ID", justify = "Justify::Right")]
    id: ActivityId,
    #[table(title = "Name")]
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

    fn activities_rows(&self) -> Vec<ActivityPertRow> {
        self.data
            .activities
            .iter()
            .map(|activity| ActivityPertRow {
                id: activity.id,
                name: activity.name.to_owned(),
                pessimistic: activity.estimation.pessimistic,
                probable: activity.estimation.probable,
                optimistic: activity.estimation.optimistic,
                pert: activity.estimated(),
            })
            .collect()
    }

    pub fn list_activities(&mut self) -> String {
        let rows: Vec<ActivityPertRow> = self.activities_rows();
        rows.with_title().display().unwrap().to_string()
    }

    pub fn pert_detail(self) -> PertReport {
        PertReport::new(self.data.pert, self.data.activities)
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
