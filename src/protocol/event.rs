use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Audio {
    AudioChunk,
    AudioStart,
    AudioStop,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Info {
    Describe,
    Info,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum SpeechRecognition {
    Transcribe,
    Transcript,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum TextToSpeech {
    Synthesize,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum WakeWord {
    Detect,
    Detection,
    NonDetected,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum VoiceActivityDetection {
    VoiceStarted,
    VoiceStopped,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum IntentRecognition {
    Recognize,
    Intent,
    NotRecognized,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum IntentHandling {
    Handled,
    NotHandled,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum AudioOutput {
    Played,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum VoiceSatellite {
    RunSatelitte,
    PauseSatelitte,
    SatelitteConnected,
    SateliteDisconnected,
    StreamingStarted,
    StreamingStopped,
    RunPipeline,
}

#[derive(Serialize, Deserialize, Debug)]
#[non_exhaustive]
#[serde(rename_all = "kebab-case")]
pub enum Timers {
    TimerStarted,
    TimerUpdated,
    TimerCancelled,
    TimerFinished,
}
