use std::task::Waker as CoreWaker;
use waker_trait::Waker;

#[derive(Clone)]
pub struct EmbassyWaker;

impl Waker for EmbassyWaker {
    fn from_waker(_waker: &CoreWaker) -> Self {
        todo!()
    }

    fn as_waker(&self) -> &CoreWaker {
        todo!()
    }

    fn wake(self) {
        todo!()
    }

    fn wake_by_ref(&self) {
        todo!()
    }
}
