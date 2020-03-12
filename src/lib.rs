#![no_std]

use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

fn test_atomic() {
    let x = AtomicBool::new(false);
    x.compare_and_swap(false, false, Ordering::Acquire);
    x.compare_and_swap(false, false, Ordering::AcqRel);
    x.compare_and_swap(false, false, Ordering::Relaxed);
    x.compare_and_swap(false, false, Ordering::Release);
    x.compare_and_swap(false, false, Ordering::SeqCst);
}

#[no_mangle]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {}
}