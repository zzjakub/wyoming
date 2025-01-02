use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub data: Option<Data>,
    pub payload: Option<Payload>,
}

impl Message {
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();

        // Convert header to JSONL, and push it to the buffer
        let header_json = serde_json::to_string(&self.header).expect("Header serialization failed");
        output.extend_from_slice(header_json.as_bytes());
        output.push(b'\n');

        // Append data if present
        if let Some(data) = &self.data {
            output.extend_from_slice(&data.0);
        }

        // Append payload if present
        if let Some(payload) = &self.payload {
            output.extend_from_slice(&payload.0);
        }

        output
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    #[serde(rename = "type")]
    pub event_type: EventType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_length: Option<BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_length: Option<BigDecimal>,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(untagged)]
pub enum EventType {
    Info(Info),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data(Vec<u8>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload(Vec<u8>);

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "lowercase")]
pub enum Info {
    Describe,
}
