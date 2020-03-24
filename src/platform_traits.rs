pub trait PlatformApplicationTrait {
    type PlatformChannel: PlatformChannelTrait + Send + Clone;
    type PlatformWaker: PlatformWakerTrait + Send + Clone;

    fn new() -> (Self::PlatformChannel, Self);

    /// Only call from the main thread.
    fn flush_events(&mut self);

    /// Only call from the main thread.
    /// On Wasm the callback is not required to be Send.
    #[cfg(not(target_arch = "wasm32"))]
    fn run<T>(&mut self, callback: T)
    where
        T: 'static + FnMut(crate::Event) + Send;

    /// Only call from the main thread.
    /// On Wasm the callback is not required to be Send.
    #[cfg(target_arch = "wasm32")]
    fn run<T>(&mut self, callback: T)
    where
        T: 'static + FnMut(crate::Event);

    fn get_waker(&self) -> Self::PlatformWaker;
}

pub trait PlatformWakerTrait {
    fn wake(&self);

    /// Should block until all events sent to the application have been processed.
    fn flush(&self);
}

/// A channel that can issue events to the main application.
pub trait PlatformChannelTrait {
    fn send(&mut self, message: crate::application_message::ApplicationMessage);
}
