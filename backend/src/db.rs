use rocket::{
    Build, Rocket,
    fairing::{self, AdHoc},
};
use rocket_db_pools::{Database, sqlx::PgPool};

#[derive(Database)]
#[database("main")]
pub(crate) struct Db(PgPool);

pub(crate) async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let Some(db) = Db::fetch(&rocket) else {
        return Err(rocket);
    };
    match sqlx::migrate!().run(&**db).await {
        Ok(_) => Ok(rocket),
        Err(e) => {
            error!("Failed to migrate database: {}", e);
            Err(rocket)
        }
    }
}

pub fn stage(migrate: bool) -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", move |rocket| async move {
        match migrate {
            true => rocket
                .attach(Db::init())
                .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations)),
            false => rocket.attach(Db::init()),
        }
    })
}
