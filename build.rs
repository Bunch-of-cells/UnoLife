fn main() {
    #[cfg(windows)]
    {
        use crate::ASSETS;
        use winres::WindowsResource;
        let path = ASSETS.join("unolife_logo.ico");
        let mut res = WindowsResource::new();
        res.set_icon(path.to_str().unwrap());
        res.compile().unwrap();
    }
}
