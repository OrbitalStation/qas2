mod c;
mod format;
mod comment;

pub use format::*;

pub fn start(code: Vec <super::token::Token>, format: Format) -> String {
    match format {
        Format::C => c::start(code).unwrap()
    }
}

pub fn uncomment(code: &mut String, format: Format) {
    match format {
        Format::C => comment::uncomment(code, &[
            comment::Comment {
                begin: "//",
                end: "\n",
                save_end: true
            },
            comment::Comment {
                begin: "/*",
                end: "*/",
                save_end: false
            }
        ])
    }
}
