use serde::Deserialize;

#[derive(Deserialize)]
pub struct RawOpcode {
    pub mnemonic: String,
    #[serde(rename = "bytes")]
    pub length: u8,
    pub cycles: Vec<u8>,
    pub operands: Vec<RawOperand>,
    pub immediate: bool,
    pub flags: RawFlags,
}

#[derive(Deserialize)]
pub struct RawOperand {
    pub name: String,
    pub immediate: bool,
    #[serde(rename = "bytes")]
    pub length: Option<u8>,
    #[serde(default)]
    pub decrement: bool,
    #[serde(default)]
    pub increment: bool,
}

#[derive(Clone, Deserialize)]
pub struct RawFlags {
    #[serde(rename = "Z")]
    pub z: String,
    #[serde(rename = "N")]
    pub n: String,
    #[serde(rename = "H")]
    pub h: String,
    #[serde(rename = "C")]
    pub c: String,
}
