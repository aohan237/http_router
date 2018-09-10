extern crate hyper;
use futures::{future, Future, Stream};
use http_router::class_view::HttpWays;
use http_router::constraint::NOTFOUND;
use http_router::url_match::UrlMatcher;
use hyper::service::service_fn;
use hyper::{Body, Chunk, Method, Request, Response, Server, StatusCode};
use std::collections::HashMap;

pub struct Router {
    pub routes: HashMap<&'static str, Box<HttpWays + Send + Sync>>,
    pub matcher: UrlMatcher,
}

impl Router {
    pub fn handle_url(
        &self,
        req: Request<Body>,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let url_matched = self.match_re_url(&req);
        let handler = self.routes.get(url_matched);
        match handler {
            Some(func) => func.dispatch(req, url_matched),
            None => {
                let body = Body::from(NOTFOUND);
                Box::new(future::ok(
                    Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(body)
                        .unwrap(),
                ))
            }
        }
    }
    pub fn register(&mut self, url: &'static str, p_view: Box<HttpWays + Send + Sync>) {
        self.matcher.add_url(url);
        self.routes.insert(url, p_view);
    }
    pub fn new() -> Router {
        Router {
            routes: HashMap::new(),
            matcher: UrlMatcher::new(),
        }
    }
    pub fn match_re_url(&self, req: &Request<Body>) -> &'static str {
        let url = req.uri().path();
        self.matcher.match_url(url)
    }
}
