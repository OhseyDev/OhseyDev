use url::Url;

pub mod templates;

pub struct Branding {
    logo: Url,
    banner: Url,
    icon: Url,
}

pub struct SiteInfo {
    name: String,
    owner: String,
    branding: Branding,
}
