use super::Pages;
impl Pages {
    pub fn construct() -> Self {
        let home: String = include_str!("../../pages/index.md").to_string();
        let install: String = include_str!("../../pages/install.md").to_string();
        let master_tutorial: String = include_str!("../../pages/master.md").to_string();
        // in order
        let pages = vec![home, install, master_tutorial];
        Self {
            current_page: 0,
            pages,
        }
    }

    pub fn current_page_content(&self) -> String {
        (*self.pages[self.current_page]).to_string()
    }
}
