use std::fs::read_to_string;

use super::Pages;
impl Pages {
    pub fn construct() -> Self {
        // in order
        let pages = vec![
            read_to_string("pages/index.md").unwrap_or_default(),
            read_to_string("pages/install.md").unwrap_or_default(),
            read_to_string("pages/master.md").unwrap_or_default(),
        ];
        Self {
            current_page: 0,
            pages,
        }
    }

    pub fn current_page_content(&self) -> String {
        (*self.pages[self.current_page]).to_string()
    }
}
