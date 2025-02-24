use macroquad::prelude::*;
use quad_files::*;
#[macroquad::main("example :>")]
async fn main() {
    let mut file: Option<FileData> = None;
    let mut dialog = FilePicker::new();
    loop {
        clear_background(BLACK);
        // update dialog, if theres data, store it in file_data
        if let FileInputResult::Data(data) = dialog.update() {
            file = Some(data);
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
        if let Some(file) = &file {
            draw_text(
                &format!("file name: {}", file.name),
                20.0,
                80.0,
                30.0,
                WHITE,
            );
            draw_text(
                &format!("last modified (unix timestamp): {}", file.timestamp),
                20.0,
                110.0,
                30.0,
                WHITE,
            );
        }
        next_frame().await
    }
}
