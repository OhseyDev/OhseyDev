
#[derive(Debug, Clone)]
pub enum Type {
    OrderedHTML(OrderedHTML),
    Markdown(String),
    Str(String)
}

#[derive(Debug, Clone)]
pub struct OrderedHTML {
    aside: Option<html::content::Aside>,
    footer: html::content::Footer,
    header: html::content::Header,
    main: html::content::Main
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Self::OrderedHTML(o) => o.to_string(),
            Self::Markdown(s) => markdown::to_html(s),
            Self::Str(s) => s
        }
    }
}

impl ToString for OrderedHTML {
    fn to_string(&self) -> String {
        let aside = if let Some(n) = self.aside { n.to_string() } else { String::new() };
        format!("{}{}{}{}", self.header.to_string(), self.main.to_string(), aside, footer)
    }
}

