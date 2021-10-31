use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Packet {
    pub(crate) packet_name: String,
    pub(crate) packet_kind: PacketKind,
    pub(crate) packet_id: usize,
    pub(crate) fields: Vec<Field>,
}

impl Packet {
    pub fn from_yaml(path: &str) -> Self {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        serde_yaml::from_str(contents.as_str()).expect("Something went wrong parsing the yaml")
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum PacketKind {
    ETCS,
    CTCS,
}

impl Into<(String, String)> for PacketKind {
    fn into(self) -> (String, String) {
        match self {
            PacketKind::ETCS => ("Etcs".to_string(), "etcs".to_string()),
            PacketKind::CTCS => ("Ctcs".to_string(), "ctcs".to_string()),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Field {
    pub(crate) var: String,
    pub(crate) len: usize,
    pub(crate) arr_len: Option<usize>,
    pub(crate) explanation: Option<String>,
    pub(crate) name: Option<String>,
}

pub(crate) fn into_multi(from: Vec<Field>) -> (Vec<String>, Vec<usize>) {
    let mut ids = Vec::new();
    let mut lens = Vec::new();
    for field in from {
        ids.push(field.var);
        lens.push(field.len);
    }
    (ids, lens)
}