#![allow(unused_variables)]

use anyhow::{anyhow, Result};

use flutter_rust_bridge::*;

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Speed {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Distance {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
#[frb(freezed)]
pub enum Measure {
    Speed(Box<Speed>),
    Distance(Box<Distance>),
}

pub fn multiply_by_ten(measure: Measure) -> Option<Measure> {
    match measure {
        Measure::Speed(b) => match *b {
            Speed::GPS(v) => Some(Measure::Speed(Box::new(Speed::GPS(v * 10.)))),
            Speed::Unknown => None,
        },
        Measure::Distance(b) => match *b {
            Distance::Map(v) => Some(Measure::Distance(Box::new(Distance::Map(v * 10.)))),
            Distance::Unknown => None,
        },
    }
}
