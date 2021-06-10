use super::keys_web;
use kapp_platform_common::*;

use std::time::Duration;

pub fn run<T>(callback: T)
where
    T: 'static + FnMut(Event),
{
}
