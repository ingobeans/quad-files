#![allow(unused_variables)]

#[cfg(target_arch = "wasm32")]
use sapp_jsutils::JsObject;

#[no_mangle]
extern "C" fn quad_files_crate_version() -> u32 {
    let major = env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor = env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();
    let patch = env!("CARGO_PKG_VERSION_PATCH").parse::<u32>().unwrap();

    (major << 24) + (minor << 16) + patch
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn quad_files_open_dialog();
    fn quad_files_read_contents() -> JsObject;
}

// for some reason, JsObject.is_nil() doesn't seem to work
// so instead i have this specific signature which means "there's no new data to be read"
const NULL_SIGNATURE: [u8; 3] = [1, 48, 90];
// signature for user cancel input
const CANCEL_SIGNATURE: [u8; 3] = [1, 48, 91];

pub fn open_dialog() {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe {
            quad_files_open_dialog();
        }
    }
}

pub enum FileInputResult {
    Canceled,
    None,
    Data(Vec<u8>),
}

pub fn read_contents() -> FileInputResult {
    #[cfg(target_arch = "wasm32")]
    {
        let file = unsafe { quad_files_read_contents() };
        let mut buf = Vec::new();
        file.to_byte_buffer(&mut buf);
        if buf == NULL_SIGNATURE {
            return FileInputResult::None;
        } else if buf == CANCEL_SIGNATURE {
            return FileInputResult::Canceled;
        }
        return FileInputResult::Data(buf);
    }
    FileInputResult::Canceled;
}
