#![allow(unused)]

use crate::prelude::*;
use std::{fmt, fs::File};
mod error;
mod prelude;
mod utils;
use colored::Colorize;
use crossbeam_channel::{unbounded, Receiver};
use std::io::ErrorKind;
use std::thread::{self, JoinHandle};

enum LightMsg {
    ChangeColor(u8, u8, u8),
    Disconnect,
    Off,
    On,
}
enum LightStatus {
    Off,
    On,
}
fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    thread::spawn(move || {
        let mut light_status = LightStatus::Off;
        loop {
            if let Ok(msg) = receiver.recv() {
                match msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!("Color changed to : {}", "     ".on_truecolor(r, g, b));
                        match light_status {
                            LightStatus::Off => println!("Light is OFF"),
                            LightStatus::On => println!("Light is ON"),
                        }
                    }
                    LightMsg::On => {
                        println!("Turned light on");
                        light_status = LightStatus::On;
                    }
                    LightMsg::Off => {
                        println!("Turned light off");
                        light_status = LightStatus::Off;
                    }
                    LightMsg::Disconnect => {
                        println!("disconnecting");
                        light_status = LightStatus::Off;
                        break;
                    }
                }
            } else {
                println!("channel disconnected");
                light_status = LightStatus::Off;
                break;
            }
        }
        light_status
    })
}
fn main() {
    let (s, r) = unbounded();
    let light = spawn_light_thread(r);
    s.send(LightMsg::On);
    s.send(LightMsg::ChangeColor(255, 0, 0));
    s.send(LightMsg::ChangeColor(25, 10, 20));
    s.send(LightMsg::ChangeColor(180, 2, 4));
    s.send(LightMsg::ChangeColor(211, 16, 0));
    s.send(LightMsg::Off);
    s.send(LightMsg::Disconnect);

    let light_status = light.join();
}
