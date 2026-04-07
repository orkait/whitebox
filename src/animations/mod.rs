pub mod alert;
pub mod angry;
pub mod curious;
pub mod focused;
pub mod guarded;
pub mod neutral;
pub mod playful;
pub mod sad;
pub mod stern;
pub mod tired;
pub mod warm;

#[derive(Debug, Clone, Copy)]
pub struct Frame {
    pub face:  &'static str,
    pub eyes:  &'static str,
    pub mouth: &'static str,
    pub ears:  &'static str,
}
