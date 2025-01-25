use actix_files as fs; 
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use mergiraf::line_merge_and_structured_resolution;
use mergiraf::settings::DisplaySettings;


#[derive(Deserialize)]
struct ResolveRequest {
    base: String,
    left: String,
    right: String,
}

#[derive(Serialize)]
struct ResolveResponse {
    result: String,
}

async fn resolve(req: web::Json<ResolveRequest>) -> impl Responder {
    // let concatenated = format!("{} {} {}", req.base, req.left, req.right);
    let merge_result = line_merge_and_structured_resolution(
        &req.base,
        &req.left,
        &req.right,
        &"temp/Base.usfm",
        &DisplaySettings::default(),
        true,
        None,
        std::option::Option::Some("./debug"),
    );
    let response = ResolveResponse {
        result: merge_result.contents,
    };
    HttpResponse::Ok().json(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // .route("/", web::get().to(|| async { "Welcome to the Rust API server!" }))
            // .route("/greet", web::post().to(greet))
            .route("/resolve", web::post().to(resolve)) // Adding the `resolve` route    })
            .service(fs::Files::new("/", "./static").index_file("index.html")) // Serve static files
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
