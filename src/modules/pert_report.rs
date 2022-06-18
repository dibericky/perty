use super::{
    activity::{Activity, EstimationValue},
    pert::Pert,
};
use anyhow::Result;
use cli_table::{format::Justify, Table, WithTitle};
use serde::Serialize;
use std::path::Path;

type Partials = liquid::partials::EagerCompiler<liquid::partials::InMemorySource>;

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

#[derive(Table, Serialize)]
pub struct PertReportRow {
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

pub struct PertReport {
    pert: Pert,
    rows: Vec<PertReportRow>,
    estimated_total: f64,
}

impl From<&Activity> for PertReportRow {
    fn from(act: &Activity) -> PertReportRow {
        Self {
            name: act.name.to_owned(),
            optimistic: act.estimation.optimistic,
            probable: act.estimation.probable,
            pessimistic: act.estimation.pessimistic,
            pert: act.estimation.estimated(),
        }
    }
}

fn estimated_total(activities: &[Activity]) -> f64 {
    activities.iter().map(|activity| activity.estimated()).sum()
}

impl PertReport {
    pub fn new(pert: Pert, activities: Vec<Activity>) -> Self {
        Self {
            pert,
            rows: activities.iter().map(|act| (act).into()).collect(),
            estimated_total: estimated_total(&activities),
        }
    }

    pub fn csv(&mut self) -> String {
        let mut csv_rows = vec![vec![
            "Name".to_string(),
            "Optimistic".to_string(),
            "Probable".to_string(),
            "Pessimistic".to_string(),
            "PERT".to_string(),
        ]];

        let mut body_rows = self
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.name.to_owned(),
                    row.optimistic.to_string(),
                    row.probable.to_string(),
                    row.pessimistic.to_string(),
                    row.pert.to_string(),
                ]
            })
            .collect::<Vec<_>>();

        csv_rows.append(&mut body_rows);
        csv_rows.push(vec![
            "Total".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            self.estimated_total.to_string(),
        ]);

        csv_rows
            .into_iter()
            .map(|vec_row| vec_row.join(","))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn html(&mut self) -> String {
        let partials = get_partials().unwrap();

        let path = Path::new("src/modules/templates/report_pert.liquid");
        let template = liquid::ParserBuilder::with_stdlib()
            .partials(partials)
            .build()
            .unwrap();

        let template = template.parse_file(path).unwrap();

        let globals = liquid::object!({
            "pert_name": self.pert.name,
            "activities": self.rows,
            "estimated_total": self.estimated_total
        });

        template.render(&globals).unwrap()
    }

    pub fn ascii(&mut self) -> String {
        let rows_str = self.rows.with_title().display().unwrap().to_string();

        format!(
            "Project: {}\n\n{}\nTOTAL: {}",
            self.pert.name, rows_str, self.estimated_total
        )
    }
}

#[cfg(test)]
mod test {
    use super::PertReport;
    use crate::modules::{activity::Activity, pert::Pert};

    #[test]
    fn table() {
        let pert = Pert::new(1, "example".to_string());
        let activities = vec![
            Activity::new(1, "activity 1".to_string(), 6, 10, 15),
            Activity::new(2, "activity 2".to_string(), 18, 25, 39),
            Activity::new(3, "activity 3".to_string(), 14, 22, 35),
            Activity::new(4, "activity 4".to_string(), 23, 34, 62),
        ];

        let mut report = PertReport::new(pert, activities);
        insta::assert_display_snapshot!(report.ascii());
    }

    #[test]
    fn table_html() {
        let pert = Pert::new(1, "example".to_string());
        let activities = vec![
            Activity::new(1, "activity 1".to_string(), 6, 10, 15),
            Activity::new(2, "activity 2".to_string(), 18, 25, 39),
            Activity::new(3, "activity 3".to_string(), 14, 22, 35),
            Activity::new(4, "activity 4".to_string(), 23, 34, 62),
        ];

        let mut report = PertReport::new(pert, activities);
        insta::assert_display_snapshot!(report.html());
    }

    #[test]
    fn table_csv() {
        let pert = Pert::new(1, "example".to_string());
        let activities = vec![
            Activity::new(1, "activity 1".to_string(), 6, 10, 15),
            Activity::new(2, "activity 2".to_string(), 18, 25, 39),
            Activity::new(3, "activity 3".to_string(), 14, 22, 35),
            Activity::new(4, "activity 4".to_string(), 23, 34, 62),
        ];

        let mut report = PertReport::new(pert, activities);
        insta::assert_display_snapshot!(report.csv());
    }
}
