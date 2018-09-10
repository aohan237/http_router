extern crate futures;
extern crate hyper;
extern crate hyper_tls;
use baidu;
use futures::{future, Future, Stream};
use http_router::constraint::{INDEX, NOTFOUND};
use hyper::client::HttpConnector;
use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode};
use serde_json;
use std::str;
use std::sync::{Arc, Mutex, MutexGuard};

pub trait HttpWays {
    fn post(
        &self,
        req: Request<Body>,
        re_url_matched: &'static str,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let body = Body::from(NOTFOUND);
        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap(),
        ))
    }
    fn get(
        &self,
        req: Request<Body>,
        re_url_matched: &'static str,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let body = Body::from(NOTFOUND);
        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap(),
        ))
    }
    fn default(&self) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let body = Body::from(NOTFOUND);
        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(body)
                .unwrap(),
        ))
    }
    fn dispatch(
        &self,
        req: Request<Body>,
        re_url_matched: &'static str,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        match req.method() {
            &Method::GET => self.get(req, re_url_matched),
            &Method::POST => self.post(req, re_url_matched),
            _ => self.default(),
        }
    }
}

pub trait ViewBuilder {
    fn new(bd: Arc<Mutex<baidu::Baidu>>) -> Self;
    fn as_view(bd: Arc<Mutex<baidu::Baidu>>) -> Box<Self>;
}
