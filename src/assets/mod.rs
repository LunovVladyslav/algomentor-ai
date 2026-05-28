use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/assets/"]
pub struct Assets;
