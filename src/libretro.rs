pub trait Host {
	fn time_nanoseconds() -> i64;
	fn perf_counter() -> u64;
}

pub struct Context {
	pub system_info: SystemInfo
}

impl Context {
	pub fn new(system_info: SystemInfo) -> Context {
		Context {
			system_info: system_info
		}
	}
}

#[repr(C)]
pub struct SystemInfo {
    pub library_name: *const u8,
    pub library_version: *const u8,
    pub valid_extensions: *const u8,
}

#[no_mangle]
pub unsafe extern fn retro_init() {

}

#[no_mangle]
pub extern fn retro_api_version() -> u32 { 1 }