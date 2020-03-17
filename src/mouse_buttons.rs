#[derive(Debug)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Extra1,
    Extra2,
    Unknown,
    __Nonexhaustive, // More buttons may be added.
}