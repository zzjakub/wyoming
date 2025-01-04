//! Rust representation of the Wyoming spec
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod event;

#[derive(Error, Debug)]
pub enum ConversionError {
    #[error("Failed to convert message header to json: {0}")]
    HeaderSerialization(#[from] serde_json::Error),
}

/// Message is used to request or supply requested data

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub data: Option<Data>,
    pub payload: Option<Payload>,
}
impl Message {
    pub fn try_into_bytes(&self) -> Result<Vec<u8>, ConversionError> {
        let mut output = Vec::new();

        // Convert header to JSONL, and push it to the buffer
        let header_json = serde_json::to_string(&self.header)?;
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

        Ok(output)
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

/// Officialy recognised event types. Their placement in the header allows for early identification
/// while data is still streaming.

// TODO: Implement streaming support

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(untagged)]
pub enum EventType {
    Audio(event::Audio),
    Info(event::Info),
    SpeechRecognition(event::SpeechRecognition),
    TextToSpeech(event::TextToSpeech),
    WakeWord(event::WakeWord),
    VoiceActivityDetection(event::VoiceActivityDetection),
    IntentRecognition(event::IntentRecognition),
    IntentHandling(event::IntentHandling),
    AudioOutput(event::AudioOutput),
    VoiceSatellite(event::VoiceSatellite),
    Timers(event::Timers),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data(Vec<u8>);

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload(Vec<u8>);
