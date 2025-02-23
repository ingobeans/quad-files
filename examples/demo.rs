use macroquad::prelude::*;
use quad_files::*;
#[macroquad::main("example :>")]
async fn main() {
    let mut file_data: Option<Vec<u8>> = None;
    let mut dialog = FilePicker::new();
    loop {
        clear_background(BLACK);
        // update dialog, if theres data, store it in file_data
        if let FileInputResult::Data(data) = dialog.update() {
            file_data = Some(data);
        }
        if is_key_pressed(KeyCode::Space) {
            dialog.open_dialog();
        }

        if is_key_pressed(KeyCode::Enter) {
            let _ = download("my_cool_file.txt", b"hello, world", Some("text file"));
        }

        draw_text("press [space] to open a file!", 20.0, 20.0, 30.0, WHITE);
        draw_text("press [enter] to download a file!", 20.0, 50.0, 30.0, WHITE);

        // if file data has been read
        if let Some(file_data) = &file_data {
            // try to parse as text
            let as_text = std::str::from_utf8(file_data);
            if let Ok(text) = as_text {
                draw_text(&format!("file text: {}", text), 20.0, 70.0, 30.0, WHITE);
            } else {
                // if not utf-8, display raw bytes
                draw_text(
                    &format!("file data: {:?}", file_data),
                    20.0,
                    70.0,
                    30.0,
                    WHITE,
                );
            }
        }
        next_frame().await
    }
}
