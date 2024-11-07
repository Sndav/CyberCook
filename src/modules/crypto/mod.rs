use strum_macros::{Display, EnumIter};

pub mod aes;

#[derive(EnumIter, Display, Default, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
enum EncryptMode {
    #[default]
    #[strum(to_string = "CBC")]
    Cbc = 0,
    #[strum(to_string = "CFB")]
    Cfb = 1,
    #[strum(to_string = "OFB")]
    Ofb = 2,
    #[strum(to_string = "CTR")]
    Ctr = 3,
    #[strum(to_string = "ECB")]
    Ecb = 4,
    #[strum(to_string = "CBC/NoPadding")]
    CbcNoPadding = 5,
    #[strum(to_string = "ECB/NoPadding")]
    EcbNoPadding = 6,
}

pub fn pkcs7_pad(input: &[u8], block_size: usize) -> anyhow::Result<Vec<u8>> {
    let padding = block_size - input.len() % block_size;
    let mut result = input.to_vec();
    result.extend(vec![padding as u8; padding]);
    Ok(result)
}

pub fn pkcs7_unpad(input: &[u8]) -> anyhow::Result<Vec<u8>> {
    let padding = input[input.len() - 1] as usize;
    if padding > input.len() {
        return Err(anyhow::anyhow!("Invalid padding"));
    }
    Ok(input[..input.len() - padding].to_vec())
}
