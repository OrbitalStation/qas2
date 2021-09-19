macro_rules! token {
    ($('ord: $($ord:ident = $ord_v:expr,)*)? $('str: $($str:ident = $str_v:expr,)*)? $('ordstr: $($ordstr:ident = $ordstr_v:expr,)*)? 'ignore: $ignore:expr) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        #[repr(u8)]
        pub enum Token {
            $($($ord,)*)?
            $($($str(String),)*)?
            $($($ordstr,)*)?
            Unknown(char),
            String(String)
        }

        impl core::fmt::Display for Token {
            fn fmt(&self, f: &mut core::fmt::Formatter <'_>) -> core::fmt::Result {
                use core::fmt::Write;

                match self {
                    $($(Self::$ord => f.write_char($ord_v),)*)?
                    $($(Self::$str(x) => f.write_str(x),)*)?
                    $($(Self::$ordstr => f.write_str($ordstr_v),)*)?
                    Self::Unknown(c) => f.write_char(*c),
                    Self::String(s) => write!(f, "\"{}\"", s)
                }
            }
        }

        impl Token {
            pub fn expect_punct(&self, token: Self) {
                if *self != token { self.panic() }
            }

            pub fn expect_value <'a> (&'a self, token: TokenStr) -> &'a String {
                if TokenStr::from_token(self) != token { self.panic() }
                match self {
                    $($(Self::$str(ref x) => x,)*)?
                    _ => unreachable!()
                }
            }

            #[cold]
            #[inline(never)]
            pub fn panic(&self) -> ! {
                panic!("`{}` wasn't expected here!", self)
            }
        }

        $(#[derive(Copy, Clone, Eq, PartialEq)]
        #[repr(u8)]
        pub enum TokenStr {
            No,
            $($str),*
        }

        impl TokenStr {
            pub fn to_token(&self, data: &String) -> Token {
                match self {
                    Self::No => unimplemented!(),
                    $(Self::$str => Token::$str(data.clone()),)*
                }
            }

            pub fn from_token(token: &Token) -> Self {
                match token {
                    $(Token::$str(_) => Self::$str,)*
                    _ => Self::No
                }
            }
        })?

        pub fn parse <I> (code: I) -> Vec <Token> where I: Iterator <Item = char> {
            let mut buf = String::new();
            let mut vec = Vec::new();
            let mut ty = TokenStr::No;

            let mut ignored = 0u8;
            let mut ignore_flush;

            for c in code {
                ignore_flush = true;
                match c {
                    $($($ord_v => push(&mut vec, &mut buf, Token::$ord, &mut ty),)*)?
                    $($(c if $str_v(c, ty) => add(&mut vec, &mut buf, &mut ty, c, TokenStr::$str),)*)?
                    c => {
                        match $ignore(c, ignored) {
                            Ok(x) => vec.push(x),
                            Err(x) => if x {
                                ignore_flush = false;
                                ignored += 1;
                                flush(&mut vec, &mut buf, &mut ty)
                            } else {
                                push(&mut vec, &mut buf, Token::Unknown(c), &mut ty)
                            }
                        }
                    }
                }
                if ignore_flush { ignored = 0 }
            }
            flush(&mut vec, &mut buf, &mut ty);

            merge_strings(&mut vec);

            vec
        }
    };
}

fn push(vec: &mut Vec <Token>, buf: &mut String, item: Token, ty: &mut TokenStr) {
    flush(vec, buf, ty);
    vec.push(item);
}

fn add(vec: &mut Vec <Token>, buf: &mut String, ty: &mut TokenStr, c: char, ty2: TokenStr) {
    if buf.is_empty() {
        *ty = ty2;
        buf.push(c);
    } else {
        if *ty != ty2 {
            vec.push(ty.to_token(buf));
            buf.clear();
        }
        *ty = ty2;
        buf.push(c);
    }
}

fn flush(vec: &mut Vec<Token>, buf: &mut String, ty: &mut TokenStr) {
    if !buf.is_empty() {
        vec.push(ty.to_token(buf));
        buf.clear()
    }
}

fn alphabetic(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

token! {
    'ord:
        LParent         = '(',
        RParent         = ')',
        Eq              = '=',
        Add             = '+',
        Sub             = '-',
        Div             = '/',
        Mul             = '*',
        Comma           = ',',
        TwoDots         = ':',
        Semicolon       = ';',
        LFigure         = '{',
        RFigure         = '}',
        LBracket        = '[',
        RBracket        = ']',
        Tab             = '\t',
        Newline         = '\n',
        Hashtag         = '#',
        Ampersand       = '&',
        DoubleQuote     = '"',

    'str:
        Name = |c: char, ty| alphabetic(c) || (ty == TokenStr::Name && c.is_numeric()),
        Number = |c: char, _| c.is_ascii_hexdigit(),

    'ordstr:
        DoubleAmpersand = "&&",

    'ignore:
        |c: char, ignored| -> Result <Token, bool> {
            if c == ' ' && ignored == 4 { Ok(Token::Tab) }
            else { Err(c.is_whitespace()) }
        }
}

pub fn merge_strings(tokens: &mut Vec<Token>) -> Option <()> {
    let mut i = 0;
    'outer: loop {
        let x = tokens.get(i);
        match x {
            Some(x) => match x {
                Token::DoubleQuote => {
                    let i_c = i;
                    let mut s = String::new();
                    tokens.remove(i);
                    loop {
                        if i >= tokens.len() { return None }
                        let c = tokens.remove(i);
                        if c == Token::DoubleQuote {
                            //tokens.remove(i);
                            tokens[i_c] = Token::String(s);
                            continue 'outer;
                        }
                        s.push_str(c.to_string().as_str());
                    }
                }
                _ => (),
            },
            None => break Some(()),
        }
        i += 1
    }
}
