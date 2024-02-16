use url::{Url, ParseError};

pub mod templates;
pub mod content;

#[derive(Debug, Clone)]
pub struct Description {
    brief: String,
    full: String,
}

impl Description {
    pub fn full(&self) -> String { self.full.clone() }
    pub fn brief(&self) -> String { self.brief.clone() }
}

impl ToString for Description {
    fn to_string(&self) { self.brief() }
}

#[derive(Debug, Clone)]
pub struct BrandInfo {
    banner: svg::Document,
    name: String,
    description: Description,
}

#[derive(Debug, Clone)]
pub struct PageInfo {
    authors: Vec<String>,
    description: Description,
    tags: Vec<String>,
    title: String,
}

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

