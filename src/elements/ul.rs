use super::HtmlElement;
use crate::{attribute::Attribute, render::Render};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Li {
    pub attributes: Vec<Attribute>,
    pub content: Rc<HtmlElement>,
}

#[derive(Debug, PartialEq)]
pub struct Ul {
    pub attributes: Vec<Attribute>,
    pub items: Vec<Li>,
}

impl Render for Li {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        let content_str = self.content.render();
        format!("<li {attr_str}>{content_str}</li>")
    }
}
impl Render for Ul {
    fn render(&self) -> String {
        let attr_str = self.attributes.render();
        let li_strs: Vec<String> = self.items.iter().map(|item| item.render()).collect();
        format!("<ul {attr_str}>\n\t{}\n</ul>", li_strs.join("\n\t"))
    }
}

impl From<Ul> for HtmlElement {
    fn from(ul: Ul) -> HtmlElement {
        HtmlElement::Ul(ul)
    }
}
