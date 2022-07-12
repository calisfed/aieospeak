#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![gen_test, sound])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn gen_test(path: &str, mut _ret: (String, String)) -> (String, String) {
    use rand::Rng;
    // use std::fs;
    // let read_part = fs::read_to_string(path).expect("error read file");
    // let part: Vec<&str> = read_part.split("\r\n\r\n").collect();
    let part: Vec<&str> = path.split("\r\n\r\n").collect();
    let rng_p = rand::thread_rng().gen_range(0..part.len());
    let part23 = part[rng_p].to_string();
    if part23.contains("PART 3:") == true {
        let sep: Vec<&str> = part23.split("PART 3").collect();
        _ret.0 = sep[0].to_string();
        _ret.1 = sep[1].to_string();
    } else {
        _ret.0 = part23.to_string();
        _ret.1 = "There is no part 3".to_string();
    }
    _ret
}
#[tauri::command]
fn sound(start: bool) -> bool {
    if start == false {
        use std::{thread, time};
        thread::sleep(time::Duration::from_secs(60));
        true
    } else {
        false
    }
}
