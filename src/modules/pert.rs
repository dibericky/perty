use anyhow::Result;

use super::{activity::Activity, storage::Storage};

pub struct Pert {
    pub name: String,
    pub activities: Box<dyn Storage>,
}

impl Pert {
    pub fn new(name: String, storage: Box<dyn Storage>) -> Self {
        Self {
            name,
            activities: storage,
        }
    }

    pub fn add_activity(&mut self, activity: Activity) -> &mut Self {
        self.activities.add_activity(&self.name, activity).unwrap();
        self
    }

    pub fn get_activities (&mut self) -> Result<Vec<&Activity>> {
        self.activities
            .get_activities(&self.name)
    }

    pub fn estimated_total(&mut self) -> f64 {
        self
            .get_activities()
            .unwrap()
            .iter()
            .map(|activity| activity.estimated())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use crate::modules::storage::{MemoryStorage};

    use super::*;

    #[test]
    fn add_activity() {
        let mut storage = MemoryStorage::new();
        let activity0 = Activity::new("activity 0".to_string(), 0, 10, 30);
        storage.add_activity("example", activity0).unwrap();

        let mut pert = Pert::new("example".to_string(), Box::new(storage));
        // let activity
        let activity = Activity::new("activity 1".to_string(), 10, 20, 30);
        pert.add_activity(activity);

        let activity0 = Activity::new("activity 0".to_string(), 0, 10, 30);
        let activity = Activity::new("activity 1".to_string(), 10, 20, 30);
        let expected = vec![
            &activity0,
            &activity
        ];

        assert_eq!(pert.name, "example".to_string());
        assert_eq!(pert.get_activities().unwrap(), expected);
    }

    #[test]
    fn estimated_total() {
        let storage = MemoryStorage::new();
        let mut pert = Pert::new("example".to_string(), Box::new(storage));
        pert.add_activity(Activity::new("activity 1".to_string(), 6, 10, 15))
            .add_activity(Activity::new("activity 2".to_string(), 18, 25, 39))
            .add_activity(Activity::new("activity 3".to_string(), 14, 22, 35))
            .add_activity(Activity::new("activity 4".to_string(), 23, 34, 62));

        assert_eq!(pert.estimated_total(), 95.99999999999999f64);
    }
}
