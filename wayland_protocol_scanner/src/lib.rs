#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub name: String,
    pub value: String,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnumChild {
    Description(String),
    Entry(Entry),
}

#[derive(Debug, Deserialize)]
pub struct Enum {
    pub name: String,
    #[serde(rename = "$value", default)]
    pub items: Vec<EnumChild>,
}

#[derive(Debug, Deserialize)]
pub struct Arg {
    pub name: String,

    #[serde(rename = "type", default)]
    pub typ: String,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventOrRequestEvent {
    Description(String),
    Arg(Arg),
}

#[derive(Debug, Deserialize)]
pub struct Event {
    pub name: String,

    #[serde(rename = "$value", default)]
    pub items: Vec<EventOrRequestEvent>,
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub name: String,

    #[serde(rename = "$value", default)]
    pub items: Vec<EventOrRequestEvent>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum InterfaceChild {
    Request(Request),
    Description(String),
    Event(Event),
    Enum(Enum),
}

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub name: String,
    pub version: String,

    #[serde(rename = "$value", default)]
    pub items: Vec<InterfaceChild>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProtocolChild {
    Interface(Interface),
    CopyRight(String),
}

#[derive(Debug, Deserialize)]
pub struct Protocol {
    pub name: String,

    #[serde(rename = "$value", default)]
    pub items: Vec<ProtocolChild>,
}

pub fn parse_wayland_protocol() -> Protocol {
    let contents = include_str!("../wayland.xml");

    from_str(&contents).unwrap()
}
