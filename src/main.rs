use std::future::Future;
use actix_web::{middleware, web::{self, Bytes}, App, HttpRequest, HttpServer};
use serde::{Deserialize, Serialize};

async fn uppercase(s: String) -> String {
    s.to_uppercase()
}

async fn handler<'a, T: Deserialize<'a> + Serialize, Fut: Future<Output = T>, F: Fn(T) -> Fut>(data: &'a Vec<u8>, func: F) -> Vec<u8> {
    let deserialized = rmp_serde::from_slice::<T>(data).unwrap();
    rmp_serde::to_vec(&func(deserialized).await).unwrap()
}

async fn server() {
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/uppercase").to(|_: HttpRequest, body: Bytes|
                async move { 
                    handler(&body.to_vec(), uppercase).await
                }
            ))
    })
    .bind(("127.0.0.1", 8080)).unwrap()
    .run()
    .await.unwrap();
}

async fn test_server() {
    let test_req = rmp_serde::to_vec(&String::from("They broke the walls we guarded.")).unwrap();
    let client = awc::Client::default();
    let res = client.post("http://127.0.0.1:8080/uppercase")
                                   .send_body(test_req)
                                   .await;
    match res {
        Ok(mut res) => {
            let body: Vec<u8> = res.body().await.unwrap().into();
            let deserialized = rmp_serde::from_slice::<String>(&body);
            
            if let Ok(returned_string) = deserialized {
                log::warn!("here comes: {:?}", returned_string);
            }
        },
        Err(e) => log::error!("{}", e),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let server_handle = tokio::spawn(server());
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    test_server().await;

    server_handle.await.unwrap();
    Ok(())
}