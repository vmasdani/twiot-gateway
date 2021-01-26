use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

pub async fn run_actix(pool: Pool<ConnectionManager<SqliteConnection>>) -> std::io::Result<()> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);
    let server_res = HttpServer::new(move || App::new().data(pool.clone()).service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    sys.await?;
    Ok(server_res)
}
