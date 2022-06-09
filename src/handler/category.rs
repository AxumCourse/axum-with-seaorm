use crate::{view, Result};

use super::{render, HtmlRespon};

pub async fn index() -> Result<HtmlRespon> {
    let handler_name = "category/index";
    let tpl = view::CategoryTemplate {};
    render(tpl, handler_name)
}
