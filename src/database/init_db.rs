use sea_orm::entity::prelude::*;
use sea_orm::{ConnectOptions, Database, DbConn};
use std::collections::HashMap;
use std::time::Duration;
use crate::models::app;

pub async fn init_database(
    app_config: &app::Config,
) -> Result<HashMap<String, DbConn>, sea_orm::DbErr> {
    let dbs_config = app_config.databases.clone();
    let mut db_map: HashMap<String, DbConn> = HashMap::new();

    for db in dbs_config {
        let db_url = format!(
            "{}://{}:{}@{}:{}/{}",
            db.database_type, db.username, db.password, db.host, db.port, db.database
        );

        let mut opt = ConnectOptions::new(&db_url);
        opt.max_connections(10)
            .min_connections(1)
            .connect_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .sqlx_logging(true);

        match Database::connect(opt).await {
            Ok(db_conn) => {
                db_map.insert(db.key.clone(), db_conn);
                crate::log_info!("{} 数据库连接成功", db.key);
            }
            Err(e) => {
                crate::log_error!("{}数据库连接失败:{}", db.key, e);
                return Err(e);
            }
        };
    }

    Ok(db_map)
}
