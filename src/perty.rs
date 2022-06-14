use anyhow::Result;

use crate::modules::{
    activity::Activity,
    pert::{Pert, PertId},
    storage::Storage,
    view::Report,
};

pub struct Perty {
    pub storage: Box<dyn Storage>,
}

impl Perty {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn add_pert(&mut self, pert_id: PertId, name: String) -> Result<()> {
        self.storage.add_pert(pert_id, &name)?;
        Ok(())
    }

    pub fn get_pert(&mut self, pert_id: PertId) -> Result<Option<Pert>> {
        self.storage.get_pert(pert_id)
    }

    pub fn get_perts(&mut self) -> Result<Vec<Pert>> {
        Ok(Vec::new())
    }

    pub fn add_activity(&mut self, pert_id: PertId, activity: Activity) -> Result<()> {
        self.storage.add_activity(pert_id, activity)?;
        Ok(())
    }

    pub fn get_activities(&mut self, pert_id: PertId) -> Result<Vec<Activity>> {
        self.storage.get_activities(pert_id)
    }

    pub fn get_reporter(&mut self, pert_id: PertId) -> Result<Report> {
        let activities = self.get_activities(pert_id)?;
        let pert = self.get_pert(pert_id)?.unwrap();
        Ok(Report::new(pert, activities))
    }
}