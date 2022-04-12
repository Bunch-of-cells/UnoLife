fn main() {
    #[cfg(windows)]
    {
        use winres::WindowsResource;
        use crate::ASSETS;
        let path = ASSETS.join("unolife_logo.ico");
        let mut res = WindowsResource::new();
        res.set_icon(path.to_str().unwrap());
        res.compile().unwrap();
    }
}
