extern crate hyper;
extern crate hyper_tls;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use hyper::rt::{self, run, Future, Stream};
use hyper::Client;
use hyper::{Body, Chunk, Error, Method, Request, Response, Server, StatusCode};
use serde_json::Value;

use chrono::prelude::*;
use hyper::client::{Builder, HttpConnector, ResponseFuture};
use std::io::{self, Write};
use std::mem;
use std::rc::Rc;
use std::string;
use std::sync::{Arc, Mutex};
#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    expires_in: i64,
}

impl Token {
    pub fn new() -> Token {
        Token {
            access_token: String::from(""),
            expires_in: 0,
        }
    }
    pub fn fresh_token(&mut self, ak: &str, sk: &str, ut: &mut i64) {
        let url = format!(
            "https://aip.baidubce.com/oauth/2.0/token?\
             grant_type=client_credentials&client_id={client_id}&\
             client_secret={client_secret}",
            client_id = ak,
            client_secret = sk
        );
        let client = reqwest::Client::new();
        let mut res = client.post(&url).send().unwrap();
        let v: Token = serde_json::from_str(&res.text().unwrap()).unwrap();
        self.access_token = v.access_token;
        self.expires_in = v.expires_in;
        *ut = Utc::now().timestamp();
    }
    pub fn get_token(&mut self, ak: &str, sk: &str, ut: &mut i64) -> String {
        let now = Utc::now().timestamp();
        if self.access_token.is_empty() {
            self.fresh_token(ak, sk, ut);
        } else if *ut + self.expires_in + 300 > now {
            self.fresh_token(ak, sk, ut);
        }
        self.access_token.clone()
    }
}

#[derive(Debug)]
pub struct Baidu {
    ak: String,
    sk: String,
    pub token: Token,
    pub update_time: i64,
    pub client: Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl Baidu {
    pub fn new(ak: &str, sk: &str) -> Baidu {
        let https = hyper_tls::HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, hyper::Body>(https);
        Baidu {
            ak: ak.to_string(),
            sk: sk.to_string(),
            token: Token::new(),
            update_time: 0,
            client: client,
        }
    }

    pub fn get_token(&mut self) -> String {
        self.token
            .get_token(self.ak.as_str(), self.sk.as_str(), &mut self.update_time)
    }
    pub fn body(&mut self) -> ResponseFuture {
        let append_url = format!("?access_token={}&charset=UTF-8", self.get_token());
        let url = String::from("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/classification/recog")
            + &append_url;
        let aa = String::from("");
        let john = json!({
            "image": aa,
            "top_num": 5,
            });
        self.client.request(
            Request::builder()
                .method(Method::POST)
                .uri(url.as_str())
                .body(john.to_string().into())
                .unwrap(),
        )
    }

    pub fn recog(&mut self, img_content: String, top_num: i32) -> ResponseFuture {
        let append_url = format!("?access_token={}&charset=UTF-8", self.get_token());
        let url = String::from("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/classification/recog")
            + &append_url;
        let data = json!({
            "image": img_content,
            "top_num": top_num,
            });
        self.client.request(
            Request::builder()
                .method(Method::POST)
                .uri(url.as_str())
                .body(data.to_string().into())
                .unwrap(),
        )
    }
    pub fn recog_proxy(&mut self, body: String) -> ResponseFuture {
        let append_url = format!("?access_token={}&charset=UTF-8", self.get_token());
        let url = String::from("https://aip.baidubce.com/rpc/2.0/ai_custom/v1/classification/recog")
            + &append_url;
        self.client.request(
            Request::builder()
                .method(Method::POST)
                .uri(url.as_str())
                .body(body.into())
                .unwrap(),
        )
    }
}
