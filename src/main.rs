#![allow(unused)]

use crate::prelude::*;
use std::{fmt, fs::read_dir};
mod error;
mod prelude;
mod utils;

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Default)]
pub struct RequestBuilder {
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}
impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder::default()
    }
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }
    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        self.method = Some(method.into());
        self
    }
    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }
    pub fn headers(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.headers.push((name.into(), value.into()));
        self
    }
    pub fn build(&self) -> Result<Request> {
        if let Some(url) = self.url.as_ref(){
            let method = self
            .method
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "GET".to_string());
        Ok(Request {
            url: url.to_string(),
            method,
            headers: self.headers.clone(),
            body: self.body.clone(),
        })
        }else{
            Err(Error::Generic("no".to_string()))
        }
       
    }
}
fn main() -> Result<()> {
    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry:?}")
    }
    let req = RequestBuilder::new()
        .url("https://some-url.com/task/123")
        .method("GET")
        .headers("token", "user_uuid.exp.sign")
        .build()?;
    println!("{req:#?}");
    Ok(())
}
