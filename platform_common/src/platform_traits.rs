pub trait PlatformApplicationTrait {
    type Channel: PlatformChannelTrait + Send + Clone;
    type Waker: PlatformWakerTrait + Send + Clone;

    fn new() -> (Self::Channel, Self);

    /// Only call from the main thread.
    fn flush_events(&mut self);

    /// Only call from the main thread.
    /// On Wasm the callback is not required to be Send.
    #[cfg(not(target_arch = "wasm32"))]
    fn run(&mut self, callback: Box<dyn FnMut(crate::Event) + Send>);

    /// Only call from the main thread.
    /// This differs from 'run' because it ensures the callback is immediately
    /// called from whatever thread the event is produced from.
    /// On Wasm the callback is not required to be Send.
    #[cfg(not(target_arch = "wasm32"))]
    fn run_raw(&mut self, callback: Box<dyn FnMut(crate::Event) + Send>);

    /// Only call from the main thread.
    /// On Wasm the callback is not required to be Send.
    #[cfg(target_arch = "wasm32")]
    fn run(&mut self, callback: Box<dyn FnMut(crate::Event)>);

    /// Only call from the main thread.
    /// On Wasm the callback is not required to be Send.
    #[cfg(target_arch = "wasm32")]
    fn run_raw(&mut self, callback: Box<dyn FnMut(crate::Event)>);

    fn get_waker(&self) -> Self::Waker;
}

pub trait PlatformWakerTrait {
    fn wake(&self);

    /// Should block until all events sent to the application have been processed.
    fn flush(&self);
}

/// A channel that can issue events to the main application.
pub trait PlatformChannelTrait {
    fn send(&mut self, message: crate::ApplicationMessage);
}
