use url::{Url, ParseError};

pub mod templates;
pub mod content;

#[derive(Debug, Clone)]
pub struct Branding {
    logo: Url,
    banner: Url,
    icon: Url
}

#[derive(Debug, Clone)]
pub struct Assets {
    stylesheet: Url,
    script: Url,
    favicon: Url,
    manifest: Url
}

#[derive(Debug, Clone)]
pub struct SiteInfo {
    assets: Assets,
    branding: Branding,
    index: Url,
    name: String,
    owner: String
}

