use core::panic::PanicInfo;

use crate::{exit, println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    match _info.location() {
        Some(location) => {
            println!(
                "Panicked at {}:{} {}",
                location.file(),
                location.line(),
                _info.message().unwrap()
            );
        }
        None => {
            println!("Panicked: {}", _info.message().unwrap());
        }
    }
    exit(1);
    unreachable!("Unreachable code in ulib panic handler!");
}
