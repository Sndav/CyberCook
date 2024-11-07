use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyInit, KeyIvInit, Unsigned};
use anyhow::anyhow;
use block_padding::{Padding, Pkcs7};
use eframe::egui::{ComboBox, Grid, Id, Ui};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::modules::{crypto::EncryptMode, input::Argument, Module};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
type Aes192CbcEnc = cbc::Encryptor<aes::Aes192>;
type Aes192CbcDec = cbc::Decryptor<aes::Aes192>;
type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;
type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
type Aes128EcbDec = ecb::Decryptor<aes::Aes128>;
type Aes192EcbEnc = ecb::Encryptor<aes::Aes192>;
type Aes192EcbDec = ecb::Decryptor<aes::Aes192>;
type Aes256EcbEnc = ecb::Encryptor<aes::Aes256>;
type Aes256EcbDec = ecb::Decryptor<aes::Aes256>;

pub struct AESEncrypt {
    id: String,
    mode: EncryptMode,
    key: Argument,
    iv: Argument,
}

pub struct AESDecrypt {
    id: String,
    mode: EncryptMode,
    key: Argument,
    iv: Argument,
}

impl Default for AESDecrypt {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            mode: EncryptMode::Cbc,
            key: Argument::default(),
            iv: Argument::default(),
        }
    }
}

impl Default for AESEncrypt {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            mode: EncryptMode::Cbc,
            key: Argument::default(),
            iv: Argument::default(),
        }
    }
}

impl Module for AESEncrypt {
    fn name(&self) -> &str {
        "AES Encrypt"
    }

    fn description(&self) -> &str {
        "Encrypt input with AES"
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn process(&self, input: &[u8]) -> anyhow::Result<Vec<u8>> {
        // encrypt
        let key = self.key.try_to_vec()?;
        let iv = self.iv.try_to_vec()?;
        match self.mode {
            EncryptMode::Cbc => {
                if key.len() == 16 && iv.len() == 16 {
                    block_encrypt_with_iv::<Aes128CbcEnc, Pkcs7>(
                        input,
                        &self.key.try_to_vec()?,
                        &self.iv.try_to_vec()?,
                    )
                } else if key.len() == 24 && iv.len() == 16 {
                    block_encrypt_with_iv::<Aes192CbcEnc, Pkcs7>(
                        input,
                        &self.key.try_to_vec()?,
                        &self.iv.try_to_vec()?,
                    )
                } else if key.len() == 32 && iv.len() == 16 {
                    block_encrypt_with_iv::<Aes256CbcEnc, Pkcs7>(
                        input,
                        &self.key.try_to_vec()?,
                        &self.iv.try_to_vec()?,
                    )
                } else {
                    Err(anyhow!("Invalid key or iv length"))
                }
            }
            EncryptMode::Ecb => {
                if key.len() == 16 {
                    block_encrypt::<Aes128EcbEnc, Pkcs7>(input, &self.key.try_to_vec()?)
                } else if key.len() == 24 {
                    block_encrypt::<Aes192EcbEnc, Pkcs7>(input, &self.key.try_to_vec()?)
                } else if key.len() == 32 {
                    block_encrypt::<Aes256EcbEnc, Pkcs7>(input, &self.key.try_to_vec()?)
                } else {
                    Err(anyhow!("Invalid key or iv length"))
                }
            }
            _ => Err(anyhow!("Unsupported mode")),
        }
    }

    fn render_inner(&mut self, ui: &mut Ui) {
        Grid::new(Id::new((&self.id, "grid")))
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Mode");
                ComboBox::from_id_salt(Id::new((&self.id, "mode")))
                    .selected_text(self.mode.to_string())
                    .show_ui(ui, |ui| {
                        for mode in EncryptMode::iter() {
                            ui.selectable_value(&mut self.mode, mode, mode.to_string());
                        }
                    });
                ui.end_row();

                ui.label("Key");
                self.key.show(ui);
                ui.end_row();

                ui.label("IV");
                self.iv.show(ui);
                ui.end_row();
            });
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(Self::default())
    }
}

impl Module for AESDecrypt {
    fn name(&self) -> &str {
        "AES Decrypt"
    }

    fn description(&self) -> &str {
        "Decrypt input with AES"
    }

    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn process(&self, input: &[u8]) -> anyhow::Result<Vec<u8>> {
        // decrypt
        let key = self.key.try_to_vec()?;
        let iv = self.iv.try_to_vec()?;
        match self.mode {
            EncryptMode::Cbc => {
                if key.len() == 16 && iv.len() == 16 {
                    block_decrypt_with_iv::<Aes128CbcDec, Pkcs7>(
                        input,
                        &self.key.try_to_vec()?,
                        &self.iv.try_to_vec()?,
                    )
                } else if key.len() == 24 && iv.len() == 16 {
                    block_decrypt_with_iv::<Aes192CbcDec, Pkcs7>(
                        input,
                        &self.key.try_to_vec()?,
                        &self.iv.try_to_vec()?,
                    )
                } else if key.len() == 32 && iv.len() == 16 {
                    block_decrypt_with_iv::<Aes256CbcDec, Pkcs7>(
                        input,
                        &self.key.try_to_vec()?,
                        &self.iv.try_to_vec()?,
                    )
                } else {
                    Err(anyhow!("Invalid key or iv length"))
                }
            }
            EncryptMode::Ecb => {
                if key.len() == 16 {
                    block_decrypt::<Aes128EcbDec, Pkcs7>(input, &self.key.try_to_vec()?)
                } else if key.len() == 24 {
                    block_decrypt::<Aes192EcbDec, Pkcs7>(input, &self.key.try_to_vec()?)
                } else if key.len() == 32 {
                    block_decrypt::<Aes256EcbDec, Pkcs7>(input, &self.key.try_to_vec()?)
                } else {
                    Err(anyhow!("Invalid key or iv length"))
                }
            }
            _ => Err(anyhow!("Unsupported mode")),
        }
    }

    fn render_inner(&mut self, ui: &mut Ui) {
        Grid::new(Id::new((&self.id, "grid")))
            .striped(true)
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Mode");
                ComboBox::from_id_salt(Id::new((&self.id, "mode")))
                    .selected_text(self.mode.to_string())
                    .show_ui(ui, |ui| {
                        for mode in EncryptMode::iter() {
                            ui.selectable_value(&mut self.mode, mode, mode.to_string());
                        }
                    });
                ui.end_row();

                ui.label("Key");
                self.key.show(ui);
                ui.end_row();

                ui.label("IV");
                self.iv.show(ui);
                ui.end_row();
            });
    }

    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(Self::default())
    }
}

fn block_encrypt<E, P>(input: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>>
where
    E: KeyInit + BlockEncryptMut,
    P: Padding<E::BlockSize>,
{
    if key.len() != E::KeySize::to_usize() {
        return Err(anyhow!("Invalid key length"));
    }

    let ct = E::new(key.into()).encrypt_padded_vec_mut::<P>(input);

    Ok(ct)
}

fn block_decrypt<E, P>(input: &[u8], key: &[u8]) -> anyhow::Result<Vec<u8>>
where
    E: KeyInit + BlockDecryptMut,
    P: Padding<E::BlockSize>,
{
    if key.len() != E::KeySize::to_usize() {
        return Err(anyhow!("Invalid key length"));
    }

    let pt = E::new(key.into())
        .decrypt_padded_vec_mut::<P>(input)
        .map_err(|e| anyhow!(e))?;

    Ok(pt.to_vec())
}

fn block_encrypt_with_iv<E, P>(input: &[u8], key: &[u8], iv: &[u8]) -> anyhow::Result<Vec<u8>>
where
    E: KeyIvInit + BlockEncryptMut,
    P: Padding<E::BlockSize>,
{
    if key.len() != E::KeySize::to_usize() || iv.len() != E::IvSize::to_usize() {
        return Err(anyhow!("Invalid key or iv length"));
    }

    let ct = E::new(key.into(), iv.into()).encrypt_padded_vec_mut::<P>(input);

    Ok(ct)
}

fn block_decrypt_with_iv<E, P>(input: &[u8], key: &[u8], iv: &[u8]) -> anyhow::Result<Vec<u8>>
where
    E: KeyIvInit + BlockDecryptMut,
    P: Padding<E::BlockSize>,
{
    if key.len() != E::KeySize::to_usize() || iv.len() != E::IvSize::to_usize() {
        return Err(anyhow!("Invalid key or iv length"));
    }

    let pt = E::new(key.into(), iv.into())
        .decrypt_padded_vec_mut::<P>(input)
        .map_err(|e| anyhow!(e))?;

    Ok(pt.to_vec())
}
