use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
pub(crate) struct Packet {
    pub(crate) name: String,
    pub(crate) kind: PacketKind,
    pub(crate) id: usize,
    pub(crate) fields: Vec<Field>,
}

impl Packet {
    pub(crate) fn from_yaml(path: &str) -> Self {
        let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
        serde_yaml::from_str(contents.as_str()).expect("Something went wrong parsing the yaml")
    }
}

#[derive(Debug, Deserialize)]
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
#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Field {
    pub(crate) variable: String,
    pub(crate) length: usize,
    pub(crate) array_len: Option<usize>,
    pub(crate) explanation: Option<String>,
    pub(crate) rename_to: Option<String>,
    pub(crate) condition: Option<Condition>,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Condition {
    pub(crate) variable: String,
    pub(crate) value: usize,
    pub(crate) operator: Operator,
}

#[derive(Clone, Debug, Deserialize)]
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
