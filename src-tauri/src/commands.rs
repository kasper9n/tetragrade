use tauri::{command, window, AppHandle, Manager, WindowUrl};

#[command]
pub fn create_child_window(app: AppHandle) {
  if app.get_window("child").is_some() {
    return ();
  }

  let main = app.get_window("main").unwrap();
  let scale_factor = main.scale_factor().unwrap();

  let main_phyiscal_pos = main.outer_position().unwrap();
  let main_pos = main_phyiscal_pos.to_logical::<i32>(scale_factor);

  let main_phyiscal_size = main.outer_size().unwrap();
  let main_size = main_phyiscal_size.to_logical::<i32>(scale_factor);

  let bar_width = 70.0;
  let margin_top = 24.0;
  let margin_bottom = 24.0;
  let margin_right = 5.0;

  let child = window::WindowBuilder::new(&app, "child", WindowUrl::default())
    .title("Child")
    .decorations(false)
    .resizable(false)
    .position(
      main_pos.x as f64 - bar_width - margin_right,
      main_pos.y as f64 + margin_top,
    )
    .inner_size(
      bar_width,
      main_size.height as f64 - margin_top - margin_bottom,
    );

  #[cfg(target_os = "macos")]
  let child = child.parent_window(main.ns_window().unwrap());
  #[cfg(windows)]
  let child = child.parent_window(main.hwnd().unwrap());

  child.build().unwrap();
}
