#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Format {
    C,
}

impl Format {
    pub fn detect(path: &str) -> Self {
        let ext = &path[path
            .rfind('.')
            .unwrap_or_else(|| panic!("file should have extension"))
            + 1..];
        assert_ne!(ext.len(), 0, "file should have extension");
        Self::only_detect(ext)
    }

    pub fn only_detect(ext: &str) -> Self {
        match ext {
            "c" | "h" => Self::C,
            ext => panic!("unknown extension: `{}`", ext),
        }
    }
}
