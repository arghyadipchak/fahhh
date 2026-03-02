use rodio::{DeviceSinkError, decoder::DecoderError};

#[derive(Debug, thiserror::Error)]
#[error("fahhh error: {0}")]
pub enum FahhhError {
    Decoder(#[from] DecoderError),
    DeviceSink(#[from] DeviceSinkError),
}
