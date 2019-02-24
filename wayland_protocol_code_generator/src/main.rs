#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

use serde::{Serialize, Deserialize};
use serde_xml_rs::from_str;
use std::fs;

#[derive(Debug, Deserialize)]
struct Entry {
    pub name: String,
    pub value: String,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum EnumChild {
    Description(String),
    Entry(Entry),
}

#[derive(Debug, Deserialize)]
struct Enum {
    pub name: String,
    #[serde(rename = "$value", default)]
    pub items: Vec<EnumChild>
}

#[derive(Debug, Deserialize)]
struct Arg {
    pub name: String,

    #[serde(rename = "type", default)]
    pub typ: String,
    pub summary: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum EventOrRequestEvent {
    Description(String),
    Arg(Arg)
}

#[derive(Debug, Deserialize)]
struct Event {
    pub name: String,

    #[serde(rename = "$value", default)]
    pub items: Vec<EventOrRequestEvent>
}

#[derive(Debug, Deserialize)]
struct Request {
    pub name: String,

    #[serde(rename = "$value", default)]
    pub items: Vec<EventOrRequestEvent>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum InterfaceChild {
    Request(Request),
    Description(String),
    Event(Event),
    Enum(Enum),
}

#[derive(Debug, Deserialize)]
struct Interface {
    pub name: String,
    pub version: String,

    #[serde(rename = "$value", default)]
    pub items: Vec<InterfaceChild>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum ProtocolChild {
    Interface(Interface),
    CopyRight(String),
}

#[derive(Debug, Deserialize)]
struct Protocol {
    pub name: String,

    #[serde(rename = "$value", default)]
    pub items: Vec<ProtocolChild>
}

fn main() {
    let contents = fs::read_to_string("wayland.xml").expect("Something went wrong reading the wayland.xml");

    let protocol: Protocol = from_str(&contents).unwrap();

    println!("{:#?}", protocol);
}
