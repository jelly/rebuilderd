use crate::errors::*;
use colored::*;
use strum_macros::{EnumString, AsRefStr, Display};
use serde::{Serialize, Deserialize};
use std::ops::Deref;
use std::str::FromStr;

pub mod api;
pub mod auth;
pub mod config;
pub mod errors;
pub mod utils;

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, AsRefStr, Serialize, Deserialize)]
#[strum(serialize_all = "kebab-case")]
#[serde(rename_all = "kebab-case")]
pub enum Distro {
    Debian,
    Archlinux,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PkgRelease {
    pub name: String,
    pub version: String,
    pub status: Status,
    pub distro: String,
    pub suite: String,
    pub architecture: String,
    pub url: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "GOOD")]
    Good,
    #[serde(rename = "BAD")]
    Bad,
    #[serde(rename = "UNKWN")]
    Unknown,
}

impl Status {
    pub fn fancy(&self) -> String {
        match self {
            Status::Good    => "GOOD ".green().to_string(),
            Status::Bad     => "BAD  ".red().to_string(),
            Status::Unknown => "UNKWN".yellow().to_string(),
        }
    }
}

impl Deref for Status {
    type Target = str;

    fn deref(&self) -> &'static str {
        match self {
            Status::Good    => "GOOD",
            Status::Bad     => "BAD",
            Status::Unknown => "UNKWN",
        }
    }
}

impl FromStr for Status {
    type Err = Error;

    fn from_str(s: &str) -> Result<Status> {
        match s {
            "GOOD" => Ok(Status::Good),
            "BAD" => Ok(Status::Bad),
            "UNKWN" => Ok(Status::Unknown),
            _ => bail!("Unknown status: {:?}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
