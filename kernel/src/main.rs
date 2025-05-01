#![no_std]
#![no_main]

use bootloader_api::{BootloaderConfig, config::Mapping, BootInfo, entry_point};

const CONFIG: BootloaderConfig = {
    let mut c = BootloaderConfig::new_default();
    c.mappings.page_table_recursive = Some(Mapping::Dynamic);
    c
};

fn start(_info: &mut BootInfo) -> ! {
    loop {}
}

entry_point!(start, config = &CONFIG);


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
