use anyhow::Result;

use super::activity::Activity;

pub trait Storage {
    fn add_activity(&mut self, pert_name: &str, activity: Activity) -> Result<()>;
    fn get_activities(&mut self, pert_name: &str) -> Result<Vec<&Activity>>;
}

#[derive(PartialEq, Debug)]
pub struct MemoryActivityRow {
    pub pert: String,
    pub activity: Activity,
}

#[derive(Default, PartialEq)]
pub struct MemoryStorage {
    pub activities: Vec<MemoryActivityRow>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self { activities: vec![] }
    }
}

impl Storage for MemoryStorage {
    fn add_activity(&mut self, pert_name: &str, activity: Activity) -> Result<()> {
        self.activities.push(MemoryActivityRow {
            pert: pert_name.to_string(),
            activity,
        });
        Ok(())
    }

    fn get_activities(&mut self, pert_name: &str) -> Result<Vec<&Activity>> {
        let activities_pert: Vec<&Activity> = self
            .activities
            .iter()
            .filter_map(|row| {
                if row.pert != pert_name {
                    return None;
                }
                Some(&row.activity)
            })
            .collect();

        Ok(activities_pert)
    }
}
