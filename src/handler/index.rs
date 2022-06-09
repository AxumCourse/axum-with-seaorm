use crate::{view, Result};

use super::{render, HtmlRespon};

pub async fn index() -> Result<HtmlRespon> {
    let handler_name = "index";
    let tpl = view::IndexTemplate {};
    render(tpl, handler_name)
}
