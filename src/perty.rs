use anyhow::Result;

use crate::modules::{
    activity::{Activity, Estimation},
    activity_report::ActivityReport,
    github::BoardId,
    pert::{Pert, PertId},
    roadmap::Roadmap,
    roadmap_report::RoadmapReport,
    storage::Storage,
};

pub struct Perty {
    pub storage: Box<dyn Storage>,
}

impl Perty {
    pub fn new(storage: Box<dyn Storage>) -> Self {
        Self { storage }
    }

    pub fn add_pert(&mut self, name: String) -> Result<PertId> {
        self.storage.add_pert(&name)
    }

    pub fn get_pert(&mut self, pert_id: PertId) -> Result<Option<Pert>> {
        self.storage.get_pert(pert_id)
    }

    pub fn get_perts(&mut self) -> Result<Vec<Pert>> {
        self.storage.get_perts()
    }

    pub fn add_activity(
        &mut self,
        pert_id: PertId,
        name: String,
        estimation: Estimation,
    ) -> Result<()> {
        self.storage.add_activity(pert_id, name, estimation)?;
        Ok(())
    }

    pub fn add_dependency(&mut self, head: PertId, tail: PertId) -> Result<()> {
        self.storage.add_dependency(head, tail)?;
        Ok(())
    }
    pub fn get_activities(&mut self, pert_id: PertId) -> Result<Vec<Activity>> {
        self.storage.get_activities(pert_id)
    }

    pub fn get_activities_reporter(&mut self, pert_id: PertId) -> Result<Option<ActivityReport>> {
        let activities = self.get_activities(pert_id)?;
        let pert = self.get_pert(pert_id)?;
        match pert {
            Some(pert) => Ok(Some(ActivityReport::new(pert, activities))),
            None => Ok(None),
        }
    }

    pub fn get_roadmap_reporter(&self, roadmap: Roadmap) -> RoadmapReport {
        RoadmapReport::new(roadmap)
    }

    pub fn get_roadmap(&mut self, pert_id: PertId) -> Result<Roadmap> {
        let acts_with_deps = self
            .storage
            .get_activities_with_related_dependencies(pert_id)?;

        Ok(Roadmap::new(acts_with_deps))
    }
    pub fn create_board(&mut self, pert_id: PertId, github_board_id: BoardId) -> Result<()> {
        self.storage.create_board(pert_id, github_board_id)?;
        Ok(())
    }
}
