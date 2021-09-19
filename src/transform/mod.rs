mod c;
mod comment;
mod format;

pub use format::*;

pub fn file() -> &'static mut String {
    static mut FILE: String = String::new();
    unsafe { &mut FILE }
}

pub fn read(path: &String) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{:?}: {}", e, path))
}

pub fn start(path: String) -> String {
    let mut code = read(&path);
    let format = Format::detect(path.as_str());
    *file() = path.clone();
    uncomment(&mut code, format);
    match format {
        Format::C => c::start(super::token::parse(code.chars())).unwrap(),
    }
}

fn uncomment(code: &mut String, format: Format) {
    match format {
        Format::C => comment::uncomment(
            code,
            &[
                comment::Comment {
                    begin: "//",
                    end: "\n",
                    save_end: true,
                },
                comment::Comment {
                    begin: "/*",
                    end: "*/",
                    save_end: false,
                },
            ],
        ),
    }
}
