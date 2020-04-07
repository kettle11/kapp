#[derive(Clone)]
pub struct WindowParameters {
    pub position: Option<(u32, u32)>,
    pub dimensions: Option<(u32, u32)>,
    pub resizable: bool,
    pub title: String,
}
