#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod menu;

fn main() {
  tauri::Builder::default()
    .menu(menu::get_menu())
    .on_menu_event(|event| {
      event.window()
          .emit("rust-event", event.menu_item_id().as_str())
          .expect("unable to emit message");
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
