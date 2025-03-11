#![allow(unused_variables)]

use std::path::PathBuf;

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
///
/// Returns [Result] holding an [Option] which on standalone will hold the path the file was saved to. This isn't available on the web and will be None.
pub fn download(
    filename: &str,
    bytes: &[u8],
    filter: Option<&str>,
) -> Result<Option<PathBuf>, std::io::Error> {
    #[cfg(target_arch = "wasm32")]
    {
        unsafe {
            let object = JsObject::buffer(bytes);
            quad_files_download(JsObject::string(filename).weak(), object.weak());
            Ok(None)
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let extension = filename.split(".").last();
        let mut dialog = rfd::FileDialog::new().set_file_name(filename);
        if let Some(extension) = extension {
            if let Some(filter) = filter {
                dialog = dialog.add_filter(filter, &[extension]);
            }
        }
        let path = dialog.save_file();
        if let Some(path) = path {
            std::fs::write(&path, bytes)?;
            Ok(Some(path))
        } else {
            Err(std::io::Error::other("File dialog was cancelled"))
        }
    }
}

#[cfg(target_arch = "wasm32")]
const NULL_STATE: u32 = 0;
#[cfg(target_arch = "wasm32")]
const CANCEL_STATE: u32 = 1;

#[derive(Clone)]
pub struct FileData {
    pub name: String,
    pub bytes: Vec<u8>,
    pub size: usize,
    pub timestamp: u64,
}

#[derive(Clone)]
pub enum FileInputResult {
    Canceled,
    None,
    Data(FileData),
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
                let file = std::fs::read(&path).unwrap();
                let metadata = std::fs::metadata(&path).unwrap();

                self.result_buffer = FileInputResult::Data(FileData {
                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                    size: file.len(),
                    bytes: file,
                    timestamp: metadata
                        .modified()
                        .unwrap()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });
            } else {
                self.result_buffer = FileInputResult::Canceled;
            }
        }
    }
    pub fn update(&mut self) -> FileInputResult {
        if self.waiting_for_result {
            #[cfg(target_arch = "wasm32")]
            {
                let contents = read_contents();
                if !matches!(contents, FileInputResult::None) {
                    self.waiting_for_result = false;
                }
                contents
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.waiting_for_result = false;
                self.result_buffer.clone()
            }
        } else {
            FileInputResult::None
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
        let js_object = unsafe { quad_files_read_contents() };

        let state = js_object.field_u32("state");
        if state == CANCEL_STATE {
            return FileInputResult::Canceled;
        } else if state == NULL_STATE {
            return FileInputResult::None;
        }

        let bytes_buf = {
            let mut buf = Vec::new();
            js_object.field("bytes").to_byte_buffer(&mut buf);
            buf
        };
        let name_buf = {
            let mut buf = String::new();
            js_object.field("name").to_string(&mut buf);
            buf
        };
        let size = {
            let mut buf = String::new();
            js_object.field("size").to_string(&mut buf);
            buf.parse::<usize>().unwrap()
        };
        let timestamp = {
            let mut buf = String::new();
            js_object.field("timestamp").to_string(&mut buf);
            buf.parse::<u64>().unwrap()
        };

        return FileInputResult::Data(FileData {
            name: name_buf,
            bytes: bytes_buf,
            size,
            timestamp,
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    FileInputResult::None
}
