use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let app = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .route("/healthz", web::get().to(|| async { "Ok! 👍🏽" }))
    })
    .bind(("0.0.0.0", 8080))?
    .run();

    println!("Running");

    app.await
}
