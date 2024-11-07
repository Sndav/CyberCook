use eframe::{egui, egui::Visuals};

use crate::{
    app::CyberCook,
    modules::{
        crypto::aes::{AESDecrypt, AESEncrypt},
        encoding::base64::{Base64Decoder, Base64Encoder},
    },
};

mod app;
mod modules;
mod views;

fn main() -> eframe::Result {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "CyberCook",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(Visuals::dark());
            Ok(Box::new(CyberCook::new(vec![
                Box::<Base64Encoder>::default(),
                Box::<Base64Decoder>::default(),
                Box::<AESEncrypt>::default(),
                Box::<AESDecrypt>::default(),
            ])))
        }),
    )
}
