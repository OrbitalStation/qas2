use super::{Token, TokenStr, Code, SmartIter};

struct Macro {
    pub name: String,
    pub data: Vec <Token>
}

impl Macro {
    pub fn macros() -> &'static mut Vec <Macro> {
        static mut MACROS: Vec <Macro> = Vec::new();
        unsafe { &mut MACROS }
    }
}

pub struct SystemPath(String);

impl SystemPath {
    pub fn new(path: &'static str) -> Self {
        let mut path = path.to_string();
        assert_eq!(path.chars().next().unwrap(), '/');
        if path.chars().last().unwrap() != '/' { path.push('/') }
        Self { 0: path }
    }

    #[cfg(target_os = "linux")]
    pub fn default() -> Vec <Self> {
        vec![Self::new("/usr/include")]
    }
}

pub fn preprocess(paths: Vec <SystemPath>) -> Option <()> {
    let mut it = SmartIter::new();
    let mut was_hashtag = false;

    loop {
        match it.next() {
            Some(x) => match x {
                Token::Name(name) => if let Some(m) = Macro::macros().iter().find(|x: &&Macro| &x.name == name) {
                    if !was_hashtag {
                        Code::code().remove(it.pos());
                        for token in m.data.iter().rev() { Code::code().insert(it.pos(), token.clone()) }
                    }
                },
                Token::Hashtag => {
                    let hashtag_pos = it.pos();
                    was_hashtag = true;
                    match it.next()? {
                        Token::Name(name) => {
                            match name.as_str() {
                                "define" => match it.next()? {
                                    Token::Name(name) => {
                                        let mut data = Vec::new();
                                        loop {
                                            let x = it.next()?;
                                            if *x == Token::Newline { break }
                                            data.push(x.clone())
                                        }
                                        Macro::macros().push(Macro {
                                            name: name.clone(),
                                            data
                                        })
                                    },
                                    x => x.panic()
                                },
                                "ifdef" => if_directive(&mut it, ifdef)?,
                                "ifndef" => if_directive(&mut it, |tokens| !ifdef(tokens))?,
                                "include" => {
                                    
                                },
                                x => panic!("unknown preprocessor directive: `{}`", x)
                            }
                            Code::code().drain(hashtag_pos..it.pos());
                            it.setpos(hashtag_pos)
                        },
                        Token::Newline => (),
                        x => x.panic()
                    }
                },
                Token::Newline => was_hashtag = false,
                _ => ()
            },
            None => break
        }
    }
    Some(())
}

fn endif(mut it: SmartIter) -> Option <usize> {
    let mut need = 1;
    loop {
        match it.next()? {
            Token::Hashtag => {
                let directive = it.next()?.expect_value(TokenStr::Name);
                if directive.starts_with("if") { need += 1 }
                else if directive == "endif" { need -= 1 }
            },
            _ => ()
        }
        if need == 0 { return Some(it.pos()) }
    }
}

fn if_directive <F> (it: &mut SmartIter, mut true_condition: F) -> Option <()> where F: FnMut(&[Token]) -> bool {
    let endif = endif(it.clone())?;
    if true_condition(&Code::code()[it.pos() + 1..it.pos() + Code::code()[it.pos()..].iter().enumerate().find(|(_, x)| **x == Token::Newline).unwrap().0]) {
        Code::code().drain(endif - 1..endif + 1);
    } else {
        it.setpos(endif + 1)
    }
    Some(())
}

fn ifdef(tokens: &[Token]) -> bool {
    if tokens.len() != 1 { panic!("too many operands") }
    else if let Token::Name(ref name) = tokens[0] { Macro::macros().iter().find(|x: &&Macro| &x.name == name).is_some() }
    else { tokens[0].panic() }
}
