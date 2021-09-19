use super::{types::TypeID, Token};

#[derive(Debug, Clone)]
pub struct Let {
    pub name: String,
    pub ty: TypeID,
}

#[derive(Clone)]
pub struct SmartIter(usize);

impl Iterator for SmartIter {
    type Item = &'static Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.check().map(|x| {
            self.0 += 1;
            x
        })
    }
}

impl SmartIter {
    #[inline]
    pub const fn new() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn pos(&self) -> usize {
        self.0 - 1
    }

    #[inline]
    pub fn setpos(&mut self, pos: usize) {
        self.0 = pos
    }

    pub fn check(&self) -> Option<<Self as Iterator>::Item> {
        Code::code().get(self.0)
    }
}

pub struct Code;

impl Code {
    pub fn set(code: Vec<Token>) {
        *Self::code() = code
    }

    pub fn code() -> &'static mut Vec<Token> {
        static mut CODE: Vec<Token> = Vec::new();
        unsafe { &mut CODE }
    }
}
