use url::Url;

pub mod templates;

pub struct Branding {
    logo: Url,
    banner: Url,
    icon: Url
}

pub struct Assets {
    stylesheet: Url,
    script: Url,
    favicon: Url,
    manifest: Url
}

pub struct SiteInfo {
    name: String,
    owner: String,
    assets: Assets,
    branding: Branding,
    index: Url
}

pub struct Content {
    header: html::content::Header,
    main: html::content::Main,
    footer: html::content::Footer
}

