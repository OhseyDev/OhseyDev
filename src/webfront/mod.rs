use url::Url;

pub mod templates;

pub struct Branding {
    logo: Url,
    banner: Url,
    icon: Url,
    stylesheet: Url,
    script: Url,
    favicon: Url,
}

pub struct SiteInfo {
    name: String,
    owner: String,
    branding: Branding,
    manifest: Url,
}

pub struct Content {
    header: String,
    main: String,
    footer: String
}

