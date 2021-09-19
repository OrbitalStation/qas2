mod ops;
mod preprocessor;
mod ty;
mod types;

use crate::token::{Token, TokenStr};
use ops::init;
pub use ty::*;
use types::Type;

pub fn start(code: Vec<Token>) -> Option<String> {
    Code::set(code);

    Type::init();

    init::init();

    preprocessor::preprocess(preprocessor::SystemPath::default());

    let mut it = SmartIter::new();
    let mut result = String::new();
    let mut lets = Vec::new();

    loop {
        match it.next() {
            Some(x) => {
                match x {
                    Token::Name(name) => {
                        if let Some(return_type) = Type::rustify(name) {
                            let name = it.next()?.expect_value(TokenStr::Name);
                            result += format!("fn {}", name).as_str();
                            match it.next()? {
                                Token::LParent => {
                                    result += "(";
                                    if *it.check()? != Token::RParent {
                                        loop {
                                            let ty = Type::trueify(
                                                it.next()?.expect_value(TokenStr::Name),
                                            );
                                            let name = it.next()?.expect_value(TokenStr::Name);
                                            result += format!(
                                                "{}: {}, ",
                                                name,
                                                Type::rustify(ty).expect("Unknown type")
                                            )
                                            .as_str();
                                            lets.push(Let {
                                                name: name.clone(),
                                                ty: Type::c2id(ty),
                                            });
                                            match it.next()? {
                                                Token::Comma => (),
                                                Token::RParent => break,
                                                x => x.panic(),
                                            }
                                        }
                                    }
                                    result.pop();
                                    result.pop();
                                    it.next()?.expect_punct(Token::LFigure);
                                    result += format!(") -> {} {{\n", return_type).as_str();
                                    loop {
                                        match it.next()? {
                                            Token::Name(x) => {
                                                result.push('\t');
                                                if *x == "return" {
                                                    let mut rop = ops::expression(&mut it, &lets)
                                                        .expect("Missing semicolon!");
                                                    let ty = Type::rust2id(return_type);
                                                    if rop.ty != ty {
                                                        rop = Let {
                                                        name: Type::convert(rop.ty, ty, &rop.name).expect(format!("`{}` is not convertible to `{}`", Type::id2c(rop.ty), Type::id2c(ty)).as_str()),
                                                        ty
                                                    }
                                                    }
                                                    result += "return ";
                                                    result.push_str(rop.name.as_str())
                                                } else {
                                                    Token::Name(x.clone()).panic()
                                                }
                                                result.push(';');
                                            }
                                            Token::Tab | Token::Newline => (),
                                            Token::RFigure => break,
                                            x => x.panic(),
                                        }
                                    }
                                    result.push_str("\n}");
                                }
                                x => x.panic(),
                            }
                        } else if *name == "typedef" {
                            let ty = Type::trueify(it.next()?.expect_value(TokenStr::Name));
                            let alias = it.next()?.expect_value(TokenStr::Name);
                            it.next()?.expect_punct(Token::Semicolon);
                            Type::add_alias(alias, Type::c2id(ty));
                            let ty = Type::rustify(ty).unwrap();
                            if alias != ty {
                                result.push_str(format!("type {} = {};\n", alias, ty).as_str())
                            }
                        }
                    }
                    Token::Newline => result.push('\n'),
                    x => x.panic(),
                }
            }
            None => break,
        }
    }

    Some(result)
}
