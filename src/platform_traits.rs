use crate::Application;
pub trait PlatformApplicationTrait {
    type PlatformChannel: PlatformChannelTrait + Clone;
    type PlatformWaker: PlatformWakerTrait + Send + Clone;

    fn new() -> (Self::PlatformChannel, Self);

    /// Only call from the main thread.
    fn flush_events(&mut self);

    /// Only call from the main thread.
    /// On Wasm the callback is not required to be Send.
    #[cfg(not(target_arch = "wasm32"))]
    fn start_receiver<T>(&mut self, application: Application, callback: T)
    where
        T: 'static + FnMut(&mut Application, crate::Event) + Send;

    /// Only call from the main thread.
    /// On Wasm the callback is not required to be Send.
    #[cfg(target_arch = "wasm32")]
    fn start_receiver<T>(&mut self, application: Application, callback: T)
    where
        T: 'static + FnMut(&mut Application, crate::Event);

    /// Only call from the main thread.
    fn start_application(self);

    fn get_waker(&self) -> Self::PlatformWaker;
}

pub trait PlatformWakerTrait {
    fn wake(&self);
}

/// A channel that can issue events to the main application.
pub trait PlatformChannelTrait {
    fn send(&mut self, message: crate::application_message::ApplicationMessage);
}
