#[cfg(windows)] use winres::WindowsResource;

 fn main() {
  if cfg!(target_os = "windows") {
    let mut res = WindowsResource::new();
    res.set_icon("src/assets/unolife_logo.ico");
    res.compile().unwrap();
  }
}