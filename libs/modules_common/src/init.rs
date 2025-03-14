use bootloader_api::BootInfo;

pub trait InitModule {
    fn init(boot_info: &mut BootInfo);
}
