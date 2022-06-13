use super::activity::Activity;

#[derive(Debug, PartialEq)]
pub struct Pert {
    pub name: String,
    pub activities: Vec<Activity>,
}

impl Pert {
    pub fn new(name: String) -> Self {
        Self {
            name,
            activities: Vec::default(),
        }
    }

    pub fn add_activity(&mut self, activity: Activity) -> &mut Self {
        self.activities.push(activity);
        self
    }

    pub fn estimated_total(&self) -> f64 {
        self.activities
            .iter()
            .map(|activity| activity.estimated())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_new_pert() {
        let pert = Pert::new("example".to_string());
        assert_eq!(
            pert,
            Pert {
                name: "example".to_string(),
                activities: vec![]
            }
        )
    }

    #[test]
    fn add_activity() {
        let mut pert = Pert::new("example".to_string());
        // let activity
        let activity = Activity::new("activity 1".to_string(), 10, 20, 30);
        pert.add_activity(activity);

        assert_eq!(
            pert,
            Pert {
                name: "example".to_string(),
                activities: vec![Activity::new("activity 1".to_string(), 10, 20, 30)]
            }
        )
    }

    #[test]
    fn estimated_total() {
        let mut pert = Pert::new("example".to_string());
        pert.add_activity(Activity::new("activity 1".to_string(), 6, 10, 15))
            .add_activity(Activity::new("activity 2".to_string(), 18, 25, 39))
            .add_activity(Activity::new("activity 3".to_string(), 14, 22, 35))
            .add_activity(Activity::new("activity 4".to_string(), 23, 34, 62));

        assert_eq!(pert.estimated_total(), 95.99999999999999f64);
    }
}
