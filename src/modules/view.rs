use super::{
    activity::{Activity, EstimationValue},
    pert::{Pert, PertId},
};
use anyhow::Result;
use cli_table::{format::Justify, Table, WithTitle};
use serde::Serialize;
use std::path::{Path};

type Partials = liquid::partials::EagerCompiler<liquid::partials::InMemorySource>;

#[derive(Table, Serialize)]
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

fn get_partials() -> Result<Partials> {
    let mut partials = Partials::empty();
    let current_dir = std::env::current_dir().unwrap();
    let relative_template_folder = "src/modules/templates/_includes".to_string();
    let partials_folder_path = current_dir.join(relative_template_folder);
    let folder_str = partials_folder_path.to_str().unwrap();
    let paths = std::fs::read_dir(&folder_str).unwrap();
    for entry in paths {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap();
        let file_path = entry.path();
        let file_path = file_path.to_str().unwrap();
        let content = std::fs::read_to_string(file_path).expect("failed to read partial file");
        partials.add(file_name, content);
    }

    Ok(partials)
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

    fn activities_rows(&self) -> Vec<Row> {
        self
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
            .collect()
    }
    pub fn table(&mut self) -> String {
        let rows: Vec<Row> = self.activities_rows();
        let rows_str = rows.with_title().display().unwrap().to_string();

        format!(
            "Project: {}\n\n{}\nTOTAL: {}",
            self.data.pert.name,
            rows_str,
            self.estimated_total()
        )
    }

    pub fn csv(&mut self) -> String {
        let rows: Vec<Row> = self.activities_rows();
        let mut csv_rows = 
            vec![
                vec!["Name".to_string(), "Optimistic".to_string(), "Probable".to_string(), "Pessimistic".to_string(), "PERT".to_string()]
            ];

        let mut body_rows = rows
            .into_iter()
            .map(|row| vec![row.name, row.optimistic.to_string(), row.probable.to_string(), row.pessimistic.to_string(), row.pert.to_string()])
            .collect::<Vec<_>>();
        
        csv_rows.append(&mut body_rows);
        csv_rows.push(vec!["Total".to_string(), "".to_string(), "".to_string(), "".to_string(), self.estimated_total().to_string()]);

        let csv = csv_rows
            .into_iter()
            .map(|vec_row| vec_row.join(","))
            .collect::<Vec<_>>()
            .join("\n");
        
        format!("{}", csv)
    }

    pub fn table_html(&mut self) -> String {
        let activities_rows: Vec<Row> = self.activities_rows();

        let partials = get_partials().unwrap();

        let path = Path::new("src/modules/templates/report_pert.liquid");
        let template = liquid::ParserBuilder::with_stdlib()
            .partials(partials)
            .build().unwrap();
        
        let template = template.parse_file(path).unwrap();

        let globals = liquid::object!({
            "pert_name": self.data.pert.name,
            "activities": activities_rows,
            "estimated_total": self.estimated_total()
        });

        let output = template.render(&globals).unwrap();
        format!("{}", output)
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

    #[test]
    fn table_html() {
        let pert = Pert::new(1, "example".to_string());
        let activities = vec![
            Activity::new("activity 1".to_string(), 6, 10, 15),
            Activity::new("activity 2".to_string(), 18, 25, 39),
            Activity::new("activity 3".to_string(), 14, 22, 35),
            Activity::new("activity 4".to_string(), 23, 34, 62),
        ];

        let mut report = Report::new(pert, activities);
        insta::assert_display_snapshot!(report.table_html());
    }

    #[test]
    fn table_csv() {
        let pert = Pert::new(1, "example".to_string());
        let activities = vec![
            Activity::new("activity 1".to_string(), 6, 10, 15),
            Activity::new("activity 2".to_string(), 18, 25, 39),
            Activity::new("activity 3".to_string(), 14, 22, 35),
            Activity::new("activity 4".to_string(), 23, 34, 62),
        ];

        let mut report = Report::new(pert, activities);
        insta::assert_display_snapshot!(report.csv());
    }
}
