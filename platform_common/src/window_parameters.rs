#[derive(Clone)]
pub struct WindowParameters {
    pub position: Option<(u32, u32)>,
    pub size: Option<(u32, u32)>,
    pub minimum_size: Option<(u32, u32)>,
    pub resizable: bool,
    pub title: String,
}
