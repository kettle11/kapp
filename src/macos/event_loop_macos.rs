use crate::Event;

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
}
