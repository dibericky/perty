use anyhow::Result;
use postgres::{Client, NoTls};

use super::{
    activity::{Activity, ActivityId, Estimation},
    pert::{Pert, PertId},
    roadmap::ActivityWithRelatedDependencies,
};

pub trait Storage {
    fn add_pert(&mut self, name: &str) -> Result<PertId>;
    fn get_pert(&mut self, pert_id: PertId) -> Result<Option<Pert>>;
    fn get_perts(&mut self) -> Result<Vec<Pert>>;
    fn add_activity(&mut self, pert_id: PertId, name: String, estimation: Estimation)
        -> Result<()>;
    fn get_activities(&mut self, pert_id: PertId) -> Result<Vec<Activity>>;
    fn add_dependency(&mut self, head: ActivityId, tail: ActivityId) -> Result<()>;
    fn get_activities_with_related_dependencies(
        &mut self,
        pert_id: PertId,
    ) -> Result<Vec<ActivityWithRelatedDependencies>>;
}

pub struct PostgresDb {
    client: Client,
}

impl PostgresDb {
    pub fn new() -> Result<Self> {
        let conn_str = std::env::var("POSTGRES_CONN_STR")?;
        let client = Client::connect(&conn_str, NoTls)?;
        Ok(Self { client })
    }
}

impl Storage for PostgresDb {
    fn add_activity(
        &mut self,
        pert_id: PertId,
        name: String,
        estimation: Estimation,
    ) -> Result<()> {
        self.client.execute(
            "INSERT INTO activities (pert_id, name, pessimistic, probable, optimistic) VALUES ($1, $2, $3, $4, $5)",
            &[
                &pert_id,
                &name,
                &estimation.pessimistic,
                &estimation.probable,
                &estimation.optimistic,
            ],
        )?;

        Ok(())
    }

    fn get_activities(&mut self, pert_id: PertId) -> Result<Vec<Activity>> {
        let rows = self.client.query(
            "
    SELECT
        pert.name as pert_name,
        pert_id,
        act.id as activity_id,
        pessimistic, probable, optimistic,
        act.name as activity_name
    FROM activities as act
    inner join pert on act.pert_id = pert.id
    WHERE pert_id = $1
	",
            &[&pert_id],
        )?;
        let activities = rows
            .into_iter()
            .map(|row| {
                Activity::new(
                    row.get("activity_id"),
                    row.get("activity_name"),
                    row.get("optimistic"),
                    row.get("probable"),
                    row.get("pessimistic"),
                )
            })
            .collect::<Vec<_>>();
        Ok(activities)
    }

    fn add_pert(&mut self, name: &str) -> Result<PertId> {
        let response = self.client.query(
            "INSERT INTO pert (name) VALUES ($1) RETURNING id as pert_id",
            &[&name],
        )?;
        let pert_id: PertId = response.get(0).unwrap().get("pert_id");

        Ok(pert_id)
    }

    fn get_pert(&mut self, pert_id: PertId) -> Result<Option<Pert>> {
        let res = match self
            .client
            .query_one("SELECT * FROM pert WHERE id = $1", &[&pert_id])
        {
            Err(_) => None,
            Ok(row) => Some(Pert::new(pert_id, row.get("name"))),
        };
        Ok(res)
    }

    fn get_perts(&mut self) -> Result<Vec<Pert>> {
        let res = self.client.query("SELECT * FROM pert", &[])?;
        let perts = res
            .iter()
            .map(|row| Pert::new(row.get("id"), row.get("name")))
            .collect();
        Ok(perts)
    }

    fn add_dependency(&mut self, head: ActivityId, tail: ActivityId) -> Result<()> {
        self.client.execute(
            "INSERT INTO activity_dependencies (activity_id_head, activity_id_tail) VALUES ($1, $2)",
            &[
                &head,
                &tail,
            ],
        )?;

        Ok(())
    }

    fn get_activities_with_related_dependencies(
        &mut self,
        pert_id: PertId,
    ) -> Result<Vec<ActivityWithRelatedDependencies>> {
        let query = "
        select 
            id as activity_id, activities.name as head_name, activity_id_head
            from activities
            full outer join activity_dependencies on activities.id  = activity_dependencies.activity_id_tail
        where pert_id = $1";
        let res = self.client.query(query, &[&pert_id])?;
        let acts = res
            .into_iter()
            .map(|row| ActivityWithRelatedDependencies {
                activity_id_head: row.get("activity_id_head"),
                head_name: row.get("head_name"),
                activity_id: row.get("activity_id"),
            })
            .collect::<Vec<_>>();
        Ok(acts)
    }
}
