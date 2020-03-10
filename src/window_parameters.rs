pub struct WindowParameters<'a> {
    pub position: Option<(u32, u32)>,
    pub dimensions: Option<(u32, u32)>,
    pub resizable: bool,
    pub title: Option<&'a str>,
}

impl<'a> Default for WindowParameters<'a> {
    fn default() -> Self {
        Self {
            position: None,
            dimensions: None,
            resizable: true,
            title: None,
        }
    }
}
