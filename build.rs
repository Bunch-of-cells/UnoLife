fn main() {
    #[cfg(windows)]
    {
        use winres::WindowsResource;
        let mut res = WindowsResource::new();
        res.set_icon("assets/unolife_logo.ico");
        res.compile().unwrap();
    }
}
