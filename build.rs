#[cfg(windows)]
fn main() {
    use winres::WindowsResource;
    let mut res = WindowsResource::new();
    res.set_icon("src/assets/unolife_logo.ico");
    res.compile().unwrap();
}

#[cfg(not(windows))]
fn main() {}
