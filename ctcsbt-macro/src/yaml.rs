use serde::{Deserialize, Serialize};
use std::fs;

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

#[derive(Default)]
pub(crate) struct FieldVecs {
    pub(crate) var_vec: Vec<String>,
    pub(crate) len_vec: Vec<usize>,
    pub(crate) arr_len_vec: Vec<Option<usize>>,
    pub(crate) explanation_vec: Vec<Option<String>>,
    pub(crate) name_vec: Vec<Option<String>>,
}

impl From<Vec<Field>> for FieldVecs {
    fn from(fields: Vec<Field>) -> Self {
        fields
            .into_iter()
            .fold(FieldVecs::default(), |mut prev, field| {
                prev.var_vec.push(field.var);
                prev.len_vec.push(field.len);
                prev.arr_len_vec.push(field.arr_len);
                prev.explanation_vec.push(field.explanation);
                prev.name_vec.push(field.name);
                prev
            })
    }
}
