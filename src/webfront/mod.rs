use url::Url;

pub mod templates;

pub struct Branding {
    logo: Url,
    banner: Url,
    icon: Url,
    assets: Url,
}

pub struct SiteInfo {
    name: String,
    owner: String,
    branding: Branding,
}

pub struct Content {
    header: String,
    main: String,
    footer: String
}

