use cli_table::{format::Justify, Table, WithTitle};
use serde::Serialize;

use super::{
    activity::ActivityId,
    roadmap::{ActivitySum, Roadmap},
};

#[derive(Table, Serialize)]
struct ActivityRow {
    #[table(title = "ID", justify = "Justify::Right")]
    id: ActivityId,
    #[table(title = "Name")]
    name: String,
}

impl From<ActivitySum> for ActivityRow {
    fn from(act: ActivitySum) -> Self {
        ActivityRow {
            id: act.id,
            name: act.name,
        }
    }
}

struct PhasesRow {
    id: usize,
    activities: Vec<ActivityRow>,
}

impl From<Roadmap> for Vec<PhasesRow> {
    fn from(roadmap: Roadmap) -> Self {
        roadmap
            .phases
            .into_iter()
            .enumerate()
            .map(|(id, ph)| {
                let activities = ph.activities.into_iter().map(|act| act.into()).collect();
                PhasesRow {
                    id: id + 1,
                    activities,
                }
            })
            .collect()
    }
}

pub struct RoadmapReport {
    phases: Vec<PhasesRow>,
}

impl RoadmapReport {
    pub fn new(roadmap: Roadmap) -> Self {
        Self {
            phases: roadmap.into(),
        }
    }

    pub fn ascii(&mut self) -> String {
        self.phases
            .iter()
            .map(|phase| {
                format!(
                    "Phase #{}\n{}",
                    phase.id,
                    phase.activities.with_title().display().unwrap()
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}
