pub mod authentication;
pub mod messages;
pub mod schema;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenvy::dotenv;
use r2d2::PooledConnection;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn get_connection(pool: DbPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get()
        .expect("Should be able to obtain db connection from pool")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_conn_str = std::env::var("DATABASE_URL").expect("DATABSES_URL must be set");
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(db_conn_str);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Database url should be correct");

    let auth_pool = pool.clone();

    let auth =
        HttpAuthentication::basic(move |x, y| authentication::validate(auth_pool.clone(), x, y));
    HttpServer::new(move || {
        App::new()
            .wrap(auth.clone())
            .app_data(web::Data::new(pool.clone()))
            .service(messages::generate_message_scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
