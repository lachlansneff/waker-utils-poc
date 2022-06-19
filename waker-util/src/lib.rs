#![no_std]

const _: () = {
    let enabled = 
        cfg!(feature = "embassy") as u32;
    
    if enabled > 1 {
        panic!("multiple waker implementations are enabled at once")
    }
};

cfg_if::cfg_if! {
    if #[cfg(feature = "embassy")] {
        pub type Waker = embassy_waker::EmbassyWaker;
    } else {
        pub type Waker = core::task::Waker;
    }
}
