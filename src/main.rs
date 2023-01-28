#![allow(unused)]

use crate::prelude::*;
use std::{fmt, fs::read_dir};
mod error;
mod prelude;
mod utils;

struct Sedan;
struct SUV;
struct Hovercraft;
trait LandCapable {
    fn drive(&self) {
        println!("Default drive")
    }
}
trait WaterCapable {
    fn sail(&self) {
        println!("Default sail")
    }
}
trait Amphibius: LandCapable + WaterCapable {}
impl LandCapable for Sedan {}
impl LandCapable for SUV {}
impl Amphibius for Hovercraft {}
impl WaterCapable for Hovercraft {
    fn sail(&self) {
        println!("Hovercraft Sailing")
    }
}
impl LandCapable for Hovercraft {
    fn drive(&self) {
        println!("Hovercraft driving")
    }
}

fn road_trip(vehicle: &impl LandCapable) {
    vehicle.drive()
}
fn traverse_frozen_lake(vehicle: &impl Amphibius) {
    vehicle.drive();
    vehicle.sail()
}
fn main() {
    let hc = Hovercraft;
    traverse_frozen_lake(&hc);
    let car = Sedan;
    road_trip(&car);
    let suv = SUV;
    road_trip(&suv)
}
