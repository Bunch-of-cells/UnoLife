fn main() {
    #[cfg(windows)]
    {
        use winres::WindowsResource;
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let mut res = WindowsResource::new();
        res.set_icon(assets.join("unolife_logo.ico"));
        res.compile().unwrap();
    }
}
