#![no_std]

use core::task::Waker as CoreWaker;

pub trait Waker: Sized + Clone {
    fn from_waker(waker: &CoreWaker) -> Self;

    fn as_waker(&self) -> &CoreWaker;
    fn wake(self);
    fn wake_by_ref(&self);
    fn will_wake(&self, other: &Self) -> bool {
        self.as_waker().will_wake(other.as_waker())
    }
}

impl Waker for CoreWaker {
    fn from_waker(waker: &CoreWaker) -> Self {
        waker.clone()
    }

    fn as_waker(&self) -> &CoreWaker {
        self
    }

    fn wake(self) {
        self.wake()
    }

    fn wake_by_ref(&self) {
        self.wake_by_ref()
    }
}
