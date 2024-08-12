use actix_web::{dev::Server, middleware, web::{self, Bytes}, App, HttpRequest, HttpServer};

use std::future::Future;
use serde::{Deserialize, Serialize};

async fn handler<'a, T, U, Fut, F>(data: &'a Vec<u8>, func: F) -> Vec<u8> 
where T: Deserialize<'a>,
      U: Serialize,
      Fut: Future<Output = U>,
      F: Fn(T) -> Fut,
{
    let deserialized = rmp_serde::from_slice::<T>(data).unwrap();
    rmp_serde::to_vec(&func(deserialized).await).unwrap()
}

pub async fn server<F, Fut>(fs: &'static std::collections::HashMap<String, F>) -> Server
where Fut: Future<Output = Vec<u8>>,
      F: 'static + Sync + Send + Clone + Fn(Vec<u8>) -> Fut,
{
    log::info!("starting HTTP server at http://localhost:8080");

    let fs = std::sync::Arc::new(fs);
    let app = move || {
        let app = App::new().wrap(middleware::Logger::default());
        fs.clone().into_iter().fold(app, |folder, (k, v)|{
            folder.service(web::resource(k).to(|_: HttpRequest, body: Bytes| {
                let f = v.clone();
                async move {
                    handler(&body.to_vec(), f).await
                }
            }))
        })
    };

    HttpServer::new(app)
    .bind(("127.0.0.1", 8080)).unwrap()
    .run()
}


pub struct SRPCServer{
    functions: std::collections::HashMap<String, Box<dyn Fn(&Vec<u8>) -> Vec<u8>>>,
}

#[derive(Debug, Clone)]
pub struct SRPCClient {}

/* impl SRPCServer {
    pub fn new() -> SRPCServer {
        SRPCServer {
            functions: HashMap::new(),
        }
    }

    pub fn handle<T: Fn(&Vec<u8>) -> Vec<u8> + 'static>(&mut self, path: String, f: T) -> &Self {
        self.functions.insert(path, Box::new(f));
        self
    }

    pub async fn bind_and_run(&self, addr: String, port: u16) -> () {
        log::info!("starting HTTP server at {}:{}", addr, port);
        let mut app = App::new().wrap(middleware::Logger::default());

        self.functions.into_iter().for_each(|(key, value)| {

            app = app.service(web::resource(key).to(|_: HttpRequest, body: Bytes| {
                async move {
                    handler(&bytes, value).await
                }
            }))
        });

        HttpServer::new(|| {
            app
        })
        .bind((addr, port)).unwrap()
        .run()
        .await.unwrap();
    }
} */