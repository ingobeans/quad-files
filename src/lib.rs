#![allow(unused_variables)]

#[cfg(target_arch = "wasm32")]
use sapp_jsutils::{JsObject, JsObjectWeak};

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
    fn quad_files_download(path: JsObjectWeak, bytes: JsObjectWeak);
}

/// Open file dialog to save the bytes to a file.
///
/// `filename` is requested file name
///
/// `bytes` is file data
///
/// if `filter` is Some, only show files of the same type in the file picker. The &str contained will be the name of the filter
pub fn download(filename: &str, bytes: &[u8], filter: Option<&str>) -> Result<(), std::io::Error> {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe {
            let object = JsObject::buffer(bytes);
            quad_files_download(JsObject::string(filename).weak(), object.weak());
            Ok(())
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let extension = filename.split(".").last();
        let mut dialog = rfd::FileDialog::new().set_file_name(filename);
        if let Some(extension) = extension {
            if let Some(filter) = filter {
                dialog = dialog.add_filter(filter, &vec![extension]);
            }
        }
        let path = dialog.save_file();
        if let Some(path) = path {
            std::fs::write(path, bytes)
        } else {
            Err(std::io::Error::other("File dialog was cancelled"))
        }
    }
}

// for some reason, JsObject.is_nil() doesn't seem to work
// so instead i have this specific signature which means "there's no new data to be read"
const NULL_SIGNATURE: [u8; 3] = [1, 48, 90];
// signature for user cancel input
const CANCEL_SIGNATURE: [u8; 3] = [1, 48, 91];

#[derive(Clone)]
pub enum FileInputResult {
    Canceled,
    None,
    Data(Vec<u8>),
}

pub struct FilePicker {
    waiting_for_result: bool,
    #[cfg(not(target_arch = "wasm32"))]
    result_buffer: FileInputResult,
}

impl FilePicker {
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            FilePicker {
                waiting_for_result: false,
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            FilePicker {
                waiting_for_result: false,
                result_buffer: FileInputResult::None,
            }
        }
    }
    pub fn open_dialog(&mut self) {
        self.waiting_for_result = true;
        #[cfg(target_arch = "wasm32")]
        {
            open_dialog();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = rfd::FileDialog::new().pick_file();
            if let Some(path) = path {
                self.result_buffer = FileInputResult::Data(std::fs::read(path).unwrap());
            } else {
                self.result_buffer = FileInputResult::Canceled;
            }
        }
    }
    pub fn update(&mut self) -> FileInputResult {
        #[cfg(target_arch = "wasm32")]
        {
            if self.waiting_for_result {
                let contents = read_contents();
                if !matches!(contents, FileInputResult::None) {
                    self.waiting_for_result = false;
                }
                contents
            } else {
                FileInputResult::None
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            if self.waiting_for_result {
                self.waiting_for_result = false;
                self.result_buffer.clone()
            } else {
                FileInputResult::None
            }
        }
    }
}

/// Opens a dialog. Contents can be read by read_contents(). Consider using a [FilePicker] instead, as they handle opnening the dialog and awaiting the result automatically.
pub fn open_dialog() {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe {
            quad_files_open_dialog();
        }
    }
}

/// Reads content from previously inputted file. Consider using a [FilePicker] instead, as they handle opnening the dialog and awaiting the result automatically.
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
    FileInputResult::Canceled
}
