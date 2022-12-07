#![allow(unused)]

use crate::prelude::*;
use std::{fmt, fs::read_dir};
mod error;
mod prelude;
mod utils;

#[derive(Debug)]
struct Task {
    name:String,
    url:String,
    headers:Vec<(String,String)>
}
#[derive(Default,Clone)]
struct BuilderTask{
    name:Option<String>,
    url:Option<String>,
    headers:Vec<(String,String)>
}
impl BuilderTask {
    fn new()-> Self{
        BuilderTask::default()
    }
    fn name(mut self, name:impl Into<String>)-> Self{
        self.name.insert(name.into());
        self
    }
    fn url(mut self,url:impl Into<String>)-> Self{
        self.url.insert(url.into());
        self
    }
    fn headers(mut self,a:impl Into<String>,b:impl Into<String>)->Self{
        self.headers.push((a.into(),b.into()));
        self
    }
    fn builder(self)->Result<Task>{
        let Some(url)= self.url else {
                return Err(Error::Generic("No url".to_string()));
        };
        Ok(Task{
            name:self.name.unwrap_or_else(||"homer".to_string()),
            url,
            headers:self.headers,
        })
    }
}
fn main() -> Result<()> {
    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry:?}")
    };
    let tasky = BuilderTask::new()
    .name("howdy")
    .url("holla.com")
    .headers("pol","cop");
    let req = tasky.clone().builder()?;
    println!("{:?}",req);
    let req = tasky.headers("Rojo","Poko").builder()?;
    println!("{:?}",req);
    Ok(())
    
}
