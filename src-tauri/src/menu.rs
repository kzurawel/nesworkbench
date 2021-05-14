use tauri::{CustomMenuItem, Menu, MenuItem};

pub fn get_menu() -> Vec<Menu<String>> {
  let new_project = MenuItem::Custom(CustomMenuItem::new("newproject".into(), "New Project"));
  #[cfg(any(target_os = "linux", target_os = "macos"))]
  let menu = {
    vec![
      Menu::new(
        "NES Workbench",
        vec![
          MenuItem::About("NES Workbench".to_string()),
          MenuItem::Services,
          MenuItem::Separator,
          MenuItem::Hide,
          MenuItem::HideOthers,
          MenuItem::ShowAll
        ],
      ),
      Menu::new(
        "File",
        vec![
          new_project
        ],
      ),
      Menu::new(
        "Edit",
        vec![
          MenuItem::Undo,
          MenuItem::Redo,
          MenuItem::Separator,
          MenuItem::Cut,
          MenuItem::Copy,
          MenuItem::Paste,
          MenuItem::Separator,
          MenuItem::SelectAll,
        ],
      ),
    ]
  };

  menu
}
