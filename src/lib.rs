pub mod config;
pub mod protocol;

pub const CONFIG_FILENAME: &str = "Config.toml";

macro_rules! apis {
    ($($name:ident => $content:expr,)*) => (
        $(#[allow(missing_docs)] pub const $name: &str = $content;)*
    )
}

apis! {
    API_URL_HEALTH => "health",
}
