use base64::Engine;
use uuid::Uuid;

use crate::modules::Module;

pub struct Base64Encoder {
    id: String,
}

pub struct Base64Decoder {
    id: String,
}

impl Default for Base64Decoder {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl Default for Base64Encoder {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
        }
    }
}

impl Module for Base64Encoder {
    fn name(&self) -> &str {
        "Base64 Encoder"
    }

    fn description(&self) -> &str {
        "Encode input to Base64"
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
    fn process(&self, input: &[u8]) -> anyhow::Result<Vec<u8>> {
        // encode
        Ok(Vec::from(
            base64::engine::general_purpose::STANDARD.encode(input),
        ))
    }
    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(Self::default())
    }
}

impl Module for Base64Decoder {
    fn name(&self) -> &str {
        "Base64 Decoder"
    }

    fn description(&self) -> &str {
        "Decode input from Base64"
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }
    fn process(&self, input: &[u8]) -> anyhow::Result<Vec<u8>> {
        Ok(base64::engine::general_purpose::STANDARD
            .decode(input)?
            .to_vec())
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(Self::default())
    }
}
