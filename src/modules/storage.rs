use anyhow::Result;
use postgres::{Client, NoTls};

use super::{
    activity::Activity,
    pert::{Pert, PertId},
};

pub trait Storage {
    fn add_pert(&mut self, pert_id: PertId, name: &str) -> Result<()>;
    fn get_pert(&mut self, pert_id: PertId) -> Result<Option<Pert>>;
    fn add_activity(&mut self, pert_id: PertId, activity: Activity) -> Result<()>;
    fn get_activities(&mut self, pert_id: PertId) -> Result<Vec<Activity>>;
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
    fn add_activity(&mut self, pert_id: PertId, activity: Activity) -> Result<()> {
        self.client.execute(
            "INSERT INTO activities (pert_id, pessimistic, probable, optimistic) VALUES ($1, $2, $3, $4)",
            &[
                &pert_id,
                &activity.estimation.pessimistic,
                &activity.estimation.probable, 
                &activity.estimation.optimistic, 
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
                    row.get("activity_name"),
                    row.get("optimistic"),
                    row.get("probable"),
                    row.get("pessimistic"),
                )
            })
            .collect::<Vec<_>>();
        Ok(activities)
    }

    fn add_pert(&mut self, pert_id: PertId, name: &str) -> Result<()> {
        self.client.execute(
            "INSERT INTO pert (id, name) VALUES ($1, $2)",
            &[&pert_id, &name],
        )?;

        Ok(())
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
}
