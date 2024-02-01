use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "basic.stpl")]
pub struct BasicPage {
    site: super::SiteInfo,
    description: String,
    title: String,
    authors: Vec<String>,
    tags: Vec<String>,
    content: super::Content
}

#[derive(TemplateOnce)]
#[template(path = "full.stpl")]
pub struct FullPage {
    site: super::SiteInfo,
    description: String,
    title: String,
    authors: Vec<String>,
    tags: Vec<String>,
    content: super::Content
}

#[derive(TemplateOnce)]
#[template(path = "brand.stpl")]
pub struct BrandPage {
    site: super::SiteInfo,
    description: String,
    title: String,
    authors: Vec<String>,
    tags: Vec<String>,
    content: super::Content
}
