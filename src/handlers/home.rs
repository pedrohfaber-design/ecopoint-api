use askama::Template;

use crate::utils::template::HtmlTemplate;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate;

pub async fn index() -> HtmlTemplate<HomeTemplate> {
    HtmlTemplate(HomeTemplate)
}
