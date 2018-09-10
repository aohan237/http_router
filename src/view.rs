use http_router::class_view::{HttpWays, ViewBuilder};
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
use baidu;
use futures::{future, Future, Stream};
// use hyper::client::HttpConnector;
use http_router::constraint::{INDEX, NOTFOUND};
use hyper::client::HttpConnector;
use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode};
use serde_json;
use std::str;
use std::sync::{Arc, Mutex, MutexGuard};
pub struct ImgRecog {
    bd: Arc<Mutex<baidu::Baidu>>,
}

impl ViewBuilder for ImgRecog {
    fn new(bd: Arc<Mutex<baidu::Baidu>>) -> ImgRecog {
        ImgRecog { bd }
    }
    fn as_view(bd: Arc<Mutex<baidu::Baidu>>) -> Box<ImgRecog> {
        Box::new(ImgRecog::new(bd))
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct ImgReq {
    img_content: String,
    top_num: i32,
}

impl HttpWays for ImgRecog {
    fn post(
        &self,
        req: Request<Body>,
        re_url_matched: &'static str,
    ) -> Box<Future<Item = Response<Body>, Error = hyper::Error> + Send> {
        let mut response = Response::new(Body::empty());
        let tt_bd = self.bd.clone();
        let reversed = req
        .into_body()
        // A future of when we finally have the full body...
        .concat2()
        // `move` the `Response` into this future...
        .map(|chunk| {
            let body = chunk.to_vec();
            let body = str::from_utf8(&body).unwrap().to_string();
            println!("{:?}", body);
            body
        })
        .and_then(
            move |body| {
                let mut tt_bd = tt_bd.lock().unwrap();
                tt_bd.recog_proxy(body)
            }
        );
        Box::new(reversed)
    }
}
