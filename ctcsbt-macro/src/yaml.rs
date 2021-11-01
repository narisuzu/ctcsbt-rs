use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Packet {
    pub(crate) name: String,
    pub(crate) kind: PacketKind,
    pub(crate) id: usize,
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

/// the field type, which represent a field in the packet
/// 
/// variable: the unique name of the field
/// length: how many bytes the field takes
/// array_len: `None` if the field is not an array, otherwise the length of the array
/// condition: `None` if the field is not conditional, otherwise the condition
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Field {
    pub(crate) variable: String,
    pub(crate) length: usize,
    pub(crate) array_len: Option<usize>,
    pub(crate) explanation: Option<String>,
    pub(crate) rename_to: Option<String>,
    pub(crate) condition: Option<Condition>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Condition {
    pub(crate) variable: String,
    pub(crate) value: usize,
    pub(crate) operator: Operator,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Operator {
    Eq,
    Gt,
    Geq,
    Lt,
    Leq,
    Neq,
}

impl Operator {
    fn calculate<T: PartialEq + PartialOrd>(&self, var: T, val: T) -> bool {
        match self {
            Operator::Eq => var == val,
            Operator::Gt => var > val,
            Operator::Lt => var < val,
            Operator::Neq => var != val,
            Operator::Geq => var >= val,
            Operator::Leq => var <= val,
        }
    }
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
                prev.var_vec.push(field.variable);
                prev.len_vec.push(field.length);
                prev.arr_len_vec.push(field.array_len);
                prev.explanation_vec.push(field.explanation);
                prev.name_vec.push(field.rename_to);
                prev
            })
    }
}
