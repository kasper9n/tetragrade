use tauri::{command, window, AppHandle, Manager, WindowUrl};

#[command]
pub fn create_child_window(id: String, app: AppHandle) {
  let main = app.get_window("main").unwrap();
  let scale_factor = main.scale_factor().unwrap();

  let main_phyiscal_pos = main.outer_position().unwrap();
  let main_pos = main_phyiscal_pos.to_logical::<i32>(scale_factor);

  let main_phyiscal_size = main.outer_size().unwrap();
  let main_size = main_phyiscal_size.to_logical::<i32>(scale_factor);

  let child = window::WindowBuilder::new(&app, id, WindowUrl::default())
    .title("Child")
    .decorations(false)
    .resizable(false)
    .position(
      // main_pos.x.into() + main_size.width - 400.0,
      // main_pos.y.into() + main_size.height - 300.0,
      main_pos.x as f64 + main_size.width as f64 - 400.0,
      main_pos.y as f64 + main_size.height as f64 - 300.0,
    )
    .inner_size(400.0, 300.0);

  #[cfg(target_os = "macos")]
  let child = child.parent_window(main.ns_window().unwrap());
  #[cfg(windows)]
  let child = child.parent_window(main.hwnd().unwrap());

  child.build().unwrap();
}
