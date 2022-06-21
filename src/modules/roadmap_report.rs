use cli_table::{format::Justify, Table, WithTitle};
use serde::Serialize;

use super::{
    activity::ActivityId,
    roadmap::{ActivitySum, Phase, Roadmap},
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

    // pub fn csv(&mut self) -> String {
    //     let mut csv_rows = vec![vec![
    //         "Name".to_string(),
    //         "Optimistic".to_string(),
    //         "Probable".to_string(),
    //         "Pessimistic".to_string(),
    //         "PERT".to_string(),
    //     ]];

    //     let mut body_rows = self
    //         .rows
    //         .iter()
    //         .map(|row| {
    //             vec![
    //                 row.name.to_owned(),
    //                 row.optimistic.to_string(),
    //                 row.probable.to_string(),
    //                 row.pessimistic.to_string(),
    //                 row.pert.to_string(),
    //             ]
    //         })
    //         .collect::<Vec<_>>();

    //     csv_rows.append(&mut body_rows);
    //     csv_rows.push(vec![
    //         "Total".to_string(),
    //         "".to_string(),
    //         "".to_string(),
    //         "".to_string(),
    //         self.estimated_total.to_string(),
    //     ]);

    //     csv_rows
    //         .into_iter()
    //         .map(|vec_row| vec_row.join(","))
    //         .collect::<Vec<_>>()
    //         .join("\n")
    // }

    // pub fn html(&mut self) -> String {
    //     let partials = get_partials().unwrap();

    //     let path = Path::new("src/modules/templates/report_pert.liquid");
    //     let template = liquid::ParserBuilder::with_stdlib()
    //         .partials(partials)
    //         .build()
    //         .unwrap();

    //     let template = template.parse_file(path).unwrap();

    //     let globals = liquid::object!({
    //         "pert_name": self.pert.name,
    //         "activities": self.rows,
    //         "estimated_total": self.estimated_total
    //     });

    //     template.render(&globals).unwrap()
    // }

    pub fn ascii(&mut self) -> String {
        self.phases
            .iter()
            .map(|phase| {
                format!(
                    "Phase #{}\n{}",
                    phase.id,
                    phase.activities.with_title().display().unwrap().to_string()
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n")
    }
}
