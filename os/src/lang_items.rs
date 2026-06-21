// 在文件最顶部添加
use crate::print;
use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let msg = info.message().as_str().unwrap_or("(no message)");

    if let Some(location) = info.location() {
        print!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            msg
        );
    } else {
        print!("Panicked: {}", msg);
    }

    shutdown()
}


