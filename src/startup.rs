//! src/lib.rs
use crate::routes;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// Notice the different signature!
// We return `Server` on the happy path and we dropped the `async` keyword
// We have no .await call, so it is not needed anymore.
pub fn run(listenser: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(routes::health_check))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(connection_pool.clone())
    })
    .listen(listenser)?
    .run();
    // No .await here!
    Ok(server)
}
