#![allow(unused)]

use crate::prelude::*;
use std::{fmt, fs::File};
mod error;
mod prelude;
mod utils;
use crossbeam_channel::{unbounded, RecvError};
use std::io::ErrorKind;
use std::thread::{self, JoinHandle};

fn main() {
    let (s, r) = unbounded();

    enum ThreadMsg {
        PrintData(String),
        Sum(i64, i64),
        Quit,
    }
    let handle = thread::spawn(move || loop {
        match r.recv() {
            Ok(msg) => match msg {
                ThreadMsg::PrintData(d) => println!("{}", d),
                ThreadMsg::Sum(lhs, rhs) => println!("{}+{}={}", lhs, rhs, (lhs + rhs)),
                ThreadMsg::Quit => {
                    println!("thread terminated");
                    break;
                }
            },
            Err(e) => {
                println!("disconnected");
                break;
            }
        }
    });
    s.send(ThreadMsg::PrintData("Hello from main".to_owned()));
    s.send(ThreadMsg::Sum(10, 10));
    s.send(ThreadMsg::Quit);
    handle.join();
}
