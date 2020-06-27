use sai::{Component, async_trait};
use bb8::{Pool, RunError};
use bb8_postgres::PostgresConnectionManager;
use std::str::FromStr;

#[derive(Component)]
#[lifecycle]
pub struct Db {
    pool: Option<Pool<PostgresConnectionManager<tokio_postgres::NoTls>>>
}

#[async_trait]
impl sai::ComponentLifecycle for Db {
    async fn start (&mut self) {
        println!("Starting DB connection...");

        let config =
            tokio_postgres::config::Config::from_str("postgresql://postgres:postgres@localhost:5432")
            .unwrap();
        let pg_mgr = PostgresConnectionManager::new(config, tokio_postgres::NoTls);

        let pool = match Pool::builder().build(pg_mgr).await {
            Ok(pool) => pool,
            Err(e) => panic!("builder error: {:?}", e),
        };

        self.pool = Some(pool);

        // Just a demo table
        match self.pool {
            Some(ref v) => {
                let client = v.get().await.unwrap();
                client.execute("CREATE TABLE IF NOT EXISTS foo ( id SERIAL )", &[]).await.unwrap();
            },
            None => {}
        }
    }

    async fn stop (&mut self) {
        // No explict action.
        // we are relying sai to drop this component automatically
        println!("Shutting down DB connections...");
    }
}

