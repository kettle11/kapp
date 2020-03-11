use crate::Application;
use crate::Event;
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;

use std::pin::Pin;
use std::task::{Context, Poll};

pub struct EventFuture<'a> {
    events: &'a mut Events,
}

impl<'a> Future for EventFuture<'a> {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context) -> Poll<Self::Output> {
        if let Some(event) = self.events.events_queue.borrow_mut().pop() {
            Poll::Ready(event)
        } else {
            Poll::Pending
        }
    }
}

impl Application {
    /// Events are sent to the program immediately as they're ready.
    /// However if they main program is blocked then events are queued.
    pub fn run_async<F>(&mut self, run_function: impl Fn(Application, Events) -> F) -> !
    where
        F: 'static + Future<Output = ()>,
    {
        let events_queue = Rc::new(RefCell::new(Vec::new()));
        let events = Events {
            events_queue: Rc::clone(&events_queue),
        };

        // Is cloning the app here problematic?
        let app_for_function = self.clone();

        let mut pin = Box::pin(run_function(app_for_function, events));

        // A proper context and waker need to be setup here.
        self.event_loop().run(move |event| {
            events_queue.borrow_mut().push(event);

            // This waker does nothing presently,
            // This means that completed futures won't actually wake up the main loop.
            // However the main loop has a chance to continue immediately on the next event.
            // In the future an artificial event should be triggered to ensure the main loop
            // continues immediately.
            // That artificial event may need to be implemented per platform.
            let waker = waker::create();
            let mut context = Context::from_waker(&waker);

            if Poll::Ready(()) == pin.as_mut().poll(&mut context) {
                // The main application loop has exited.
                // Do something here!
            }
        });
        unreachable!()
    }
}

pub struct Events {
    events_queue: Rc<RefCell<Vec<Event>>>,
}

impl Events {
    pub fn next_event(&mut self) -> self::EventFuture {
        self::EventFuture { events: self }
    }
}

mod waker {
    use std::task::{RawWaker, RawWakerVTable, Waker};

    pub fn create() -> Waker {
        unsafe { Waker::from_raw(RAW_WAKER) }
    }

    const RAW_WAKER: RawWaker = RawWaker::new(std::ptr::null(), &VTABLE);
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe fn clone(_: *const ()) -> RawWaker {
        RAW_WAKER
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}
}
