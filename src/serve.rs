use actix_web::{HttpServer, Responder, web, App};

async fn hello_world() -> impl Responder {
    "Hello, world!"
}

pub async fn run_actix() -> std::io::Result<()> {
    let local = tokio::task::LocalSet::new();
    let sys = actix_rt::System::run_in_tokio("server", &local);
    let server_res = HttpServer::new(|| App::new().route("/", web::get().to(hello_world)))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;

    sys.await?;
    Ok(server_res)
}