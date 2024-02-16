use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "basic.stpl")]
pub struct BasicPage {
    content: super::Content,
    page: super::PageInfo,
    site: super::SiteInfo,
}

#[derive(TemplateOnce)]
#[template(path = "full.stpl")]
pub struct FullPage {
    content: super::Content,
    page: super::PageInfo,
    site: super::SiteInfo,
}

#[derive(TemplateOnce)]
#[template(path = "brand.stpl")]
pub struct BrandPage {
    brand: super::BrandInfo,
    content: super::Content,
    page: super::PageInfo,
    site: super::SiteInfo,
}

