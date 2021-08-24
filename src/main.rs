
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};


use winapi::{
    um::{
        winuser::{
          EnumWindows, GetWindowTextW, GetWindowTextLengthW, IsWindowVisible,
          GetWindowRect, SetWindowPos,
        },
        winnt::LPWSTR
    },
    shared::{minwindef::{BOOL, LPARAM}, windef::HWND},
};

// I don't mind paying a vtable for this sugar.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;


fn main() {
  let exit_f: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
  intelligently_hide_ms_console();
  
  let bg_exit_f = exit_f.clone();
  thread::spawn(move || {
    let top_buffer: String = env::var("MISC_WFIX_top_buffer").unwrap_or("50".to_string());
    let top_buffer: isize = top_buffer.parse().unwrap_or(50);
    eprintln!("Using top_buffer={}", top_buffer);
    loop {
      if bg_exit_f.load(Ordering::SeqCst) {
        break;
      }
      run_bg(top_buffer);
    }
  });

  if let Err(e) = run_gui(exit_f.clone()) {
    eprintln!("{}", e);
  }

  exit_f.store(true, Ordering::SeqCst);

}

fn run_gui(bg_exit_f: Arc<AtomicBool>) -> Result<()> {
  let mut a = systray::Application::new()?;

  a.add_menu_item("Misc-Winfixes is running", |_window| {
      Ok::<_, systray::Error>(())
  })?;

  a.add_menu_item("Quit", move |window| {
      bg_exit_f.store(true, Ordering::SeqCst);
      window.quit();
      Ok::<_, systray::Error>(())
  })?;

  a.wait_for_message()?;

  Ok(())
}

fn run_bg(top_buffer: isize, ) {
  // Poll for windows with y < 50


  eprintln!("===============");
  for w in window_data().unwrap_or(vec![]) {
    eprintln!("w={:?}", w);
    if w.top < top_buffer {
      // Move window down to top_buffer pixels
      eprintln!("Resizing {}", &w.name);

      unsafe {
        // https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos
        winapi::um::winuser::SetWindowPos(
          w.hwnd, winapi::um::winuser::HWND_TOP,
          w.left as i32, top_buffer as i32, (w.right - w.left) as i32, (w.bot - top_buffer) as i32,
          std::mem::zeroed(),
        );
      }

    }
  }


  thread::sleep(time::Duration::from_millis(340));
}


fn intelligently_hide_ms_console() {
    // Check if we are run from the console or just launched with explorer.exe
    let mut console_proc_list_buff: Vec<u32> = vec![0; 16];
    let num_procs = unsafe {
        winapi::um::wincon::GetConsoleProcessList(console_proc_list_buff.as_mut_ptr(), 16)
    };
    println!("num_procs={:?}", num_procs);
    if num_procs == 1 {
        // We were launched from explorer.exe, detatch the console
        unsafe { winapi::um::wincon::FreeConsole() };
    }
    // Otherwise do nothing, we want console messages when run from the console.
}

#[derive(Debug, Clone)]
struct WindowData {
  pub name: String,
  pub left: isize,
  pub top: isize,
  pub right: isize,
  pub bot: isize,
  pub hwnd: HWND,
}

fn window_data() -> Result<Vec<WindowData>> {
      let state: Box<Vec<WindowData>> = Box::new(Vec::new());
      let ptr = Box::into_raw(state);
      let state;
      unsafe {
          EnumWindows(Some(enumerate_windows), ptr as LPARAM);
          state = Box::from_raw(ptr);
      }
      Ok(*state)
  }


unsafe extern "system" fn enumerate_windows(window: HWND, state: LPARAM) -> BOOL {
    // Ignore hidden windows
    if IsWindowVisible(window) == 0 {
      return true.into()
    }

    let state = state as *mut Vec<WindowData>;
    let mut w = WindowData {
      name: String::new(),
      left: 0,
      top: 0,
      right: 0,
      bot: 0,
      hwnd: window.clone(),
    };

    // Get title
    let length = GetWindowTextLengthW(window);
    let mut title: Vec<u16> = vec![0; length as usize];
    if GetWindowTextW(window, title.as_mut_ptr() as LPWSTR, length+1) != 0 {
        if let Ok(title) = String::from_utf16(title[0..(length as usize)].as_ref()) {
            w.name = title;
        }
    }

    // Get y from rect
    let mut r = std::mem::zeroed::<winapi::shared::windef::RECT>();
    GetWindowRect(window, &mut r);
    w.left = r.left as isize;
    w.top = r.top as isize;
    w.right = r.right as isize;
    w.bot = r.bottom as isize;

    // Ignore windows smaller than 400px in at least one dimension
    if (w.right - w.left) < 400 || (w.bot - w.top) < 400 {
      return true.into();
    }


    // Finally return to owner
    (*state).push(w);

    true.into()
}
