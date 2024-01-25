use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "basic.stpl")]
pub struct BasicPage {
    site: super::SiteInfo,
    description: String,
    title: String,
    authors: Vec<String>,
    tags: Vec<String>
}

