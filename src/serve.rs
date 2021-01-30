use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

use handler::*;

use crate::handler;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn run_actix(pool: Pool<ConnectionManager<SqliteConnection>>) -> std::io::Result<()> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);
    let server_res = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(index)
            // Schedule
            .service(all_schedules)
            .service(view_schedules)
            .service(post_schedule)
            .service(get_schedule)
            .service(delete_schedule)
            // Watering time
            .service(all_wateringtimes)
            .service(post_wateringtime)
            .service(get_wateringtime)
            .service(delete_wateringtime)
            // Devices
            .service(all_devices)
            // MAC Registration
            .service(register_device)
            // Misc
            .service(check_resp)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    sys.await?;
    Ok(server_res)
}
