use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn demo() -> impl Responder {
    let _ = get_ip().await;
    HttpResponse::Ok().body("Hello,Rust")
}

async fn get_ip() -> reqwest::Result<()> {
    let resp = reqwest::Client::new().get("https://httpbin.org/ip")
        .send().await?.text().await?;
    println!("{}", resp);
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(demo)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
