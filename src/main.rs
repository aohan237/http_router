mod baidu;
mod http_router;
mod view;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;
extern crate tokio;

use futures::{future, Future, Stream};
// use hyper::client::HttpConnector;
use http_router::class_view::{HttpWays, ViewBuilder};
use http_router::router::Router;
use hyper::service::service_fn;
use hyper::{Body, Chunk, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::prelude::*;
use tokio::timer::Interval;
use view::ImgRecog;

fn main() {
    pretty_env_logger::init();
    let addr = "127.0.0.1:1337".parse().unwrap();

    hyper::rt::run(future::lazy(move || {
        let mut bd = baidu::Baidu::new("test", "test");

        let mut bd = Arc::new(Mutex::new(bd));
        let mut c_router = Router::new();
        // c_router.register("/test", Index::as_view(bd.clone()));
        // c_router.register("/state", Index::as_view(bd.clone()));
        c_router.register(r"^/(?x)(?P<year>\d{4})", ImgRecog::as_view(Arc::clone(&bd)));
        c_router.register(
            r"^/year/(?x)(?P<year>\d{4})",
            ImgRecog::as_view(Arc::clone(&bd)),
        );
        let s2 = Arc::new(c_router);

        let new_service = move || {
            // println!("calling service");
            let tt_router = s2.clone();
            service_fn(move |req| tt_router.handle_url(req))
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        server
    }));
}
