///
/// @package Showcase-Microservices-Rust
///
/// @file Main entry
/// @copyright 2024-present Christoph Kappel <christoph@unexist.dev>
/// @version $Id$
///
/// This program can be distributed under the terms of the GNU GPLv2.
/// See the file LICENSE for details.
///

use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    title: String,
    description: String,
}

async fn index(todo: web::Json<Todo>) -> HttpResponse {
    println!("Model: {:?}", &todo);
    HttpResponse::Ok().json(todo.0)
}

async fn create_todo(todo: web::Json<Todo>, req: HttpRequest) -> HttpResponse {
    println!("Request: {req:?}");
    println!("Model: {todo:?}");

    HttpResponse::Ok().json(todo.0)
}

async fn index_manual(body: web::Bytes) -> Result<HttpResponse, Error> {
    let obj = serde_json::from_slice::<Todo>(&body)?;
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/todo").route(web::get().to(index)))
            .service(
                web::resource("/todo")
                    .app_data(web::JsonConfig::default().limit(1024)) // <- limit size of the payload (resource level)
                    .route(web::post().to(create_todo)),
            )
            .service(web::resource("/manual").route(web::post().to(index_manual)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{body::to_bytes, dev::Service, http, test};

    use super::*;

    #[actix_web::test]
    async fn test_index() {
        let app =
            test::init_service(App::new().service(web::resource("/todo").route(web::post().to(index))))
                .await;

        let req = test::TestRequest::post()
            .uri("/todo")
            .set_json(Todo {
                title: "string".to_owned(),
                description: "string".to_owned(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        assert_eq!(body_bytes, r#"{"title":"string","description":"string"}"#);
    }
}
