use macroquad::prelude::*;
use quad_files::*;
#[macroquad::main("example :>")]
async fn main() {
    let mut file_data: Vec<u8> = Vec::new();
    let mut dialog_open = false;
    loop {
        clear_background(BLACK);
        if file_data.is_empty() && dialog_open {
            let data = read_contents();
            if let Some(data) = data {
                dialog_open = false;
                file_data = data;
            }
        }
        if is_key_pressed(KeyCode::Space) {
            open_dialog();
            dialog_open = true;
        }

        draw_text("press [space] to select a file!", 20.0, 20.0, 30.0, WHITE);
        draw_text(&format!("file {:?}", file_data), 20.0, 70.0, 30.0, WHITE);
        next_frame().await
    }
}
