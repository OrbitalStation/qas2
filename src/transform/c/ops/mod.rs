pub use types::*;

use super::{types::TypeID, Let, SmartIter, Type};
use crate::token::Token;

pub mod init;
mod types;

fn number_ty(v: &String) -> TypeID {
    let n: i128 = v.parse().unwrap();
    if n >= -128 && n <= 128 {
        Type::c2id("signed char")
    } else if n >= -32768 && n <= 32767 {
        Type::c2id("short")
    } else if n >= -2147483648 && n <= 2147483647 {
        Type::c2id("int")
    } else if n < 0 {
        Type::c2id("long")
    } else if n <= 255 {
        Type::c2id("unsigned char")
    } else if n <= 65535 {
        Type::c2id("unsigned short")
    } else if n <= 4294967295 {
        Type::c2id("unsigned int")
    } else {
        Type::c2id("unsigned long")
    }
}

fn eval_one_expr(
    operands: &Vec<Operand>,
    operators: &Vec<Operator>,
    i: &Op,
) -> Option<Operand<'static>> {
    'outer: for k in if i.order == Order::Left {
        0..if operands.len() == 1 {
            1
        } else {
            operands.len() as isize - 1
        }
    } else {
        operands.len() as isize - 1..-1
    } {
        let k = k as usize;

        if i.operands.len() == 1 && i.fix != operators[k].fix {
            continue;
        }

        for kk in 0..i.operands.len() {
            match i.parts.get(kk) {
                Some(x) => {
                    if *x != operators[k + kk].data {
                        continue 'outer;
                    }
                }
                None => (),
            }
            match operands.get(k + kk) {
                Some(x) => {
                    if i.operands[kk] != x.ty() {
                        continue 'outer;
                    }
                }
                None => continue 'outer,
            }
        }

        return Some(Operand::Expr(Let {
            name: (i.make)(&{
                let mut args = Vec::new();
                for kk in 0..i.operands.len() {
                    args.push(operands[k + kk].as_let())
                }
                args
            }),
            ty: i.ret,
        }));
    }

    None
}

pub fn expression(it: &mut SmartIter, lets: &Vec<Let>) -> Option<Let> {
    let mut operands = Vec::new();
    let mut operators = Vec::new();

    let mut prev_was_operator = true;
    loop {
        match it.next()? {
            Token::Semicolon => break,
            Token::Name(name) => {
                for i in lets {
                    if i.name == name.as_str() {
                        operands.push(Operand::Let(i));
                    }
                }
                prev_was_operator = false
            }
            Token::Number(ref value) => {
                operands.push(Operand::Expr(Let {
                    name: value.clone(),
                    ty: number_ty(value),
                }));
                prev_was_operator = false
            }
            x => operators.push(Operator {
                data: x.to_string(),
                fix: if prev_was_operator {
                    Fix::Prefix
                } else {
                    Fix::Postfix
                },
            }),
        }
    }

    'outer: for i in Op::ops().iter() {
        loop {
            if operators.len() == 0 {
                break 'outer;
            }

            let result = match eval_one_expr(&operands, &operators, i) {
                Some(x) => x,
                None => break,
            };

            match i.order {
                Order::Left => {
                    unsafe {
                        *(&mut operands as *mut Vec<Operand>) = operands
                            .clone()
                            .into_iter()
                            .skip(i.operands.len())
                            .collect()
                    }
                    operators.remove(0);
                    operands.insert(0, result)
                }
                Order::Right => {
                    operands.truncate(operands.len() - i.operands.len());
                    operators.pop();
                    operands.push(result)
                }
            }
        }
    }

    if !operators.is_empty() || operands.len() != 1 {
        panic!("unknown operator or there's no overloads for this operator")
    }

    Some(operands[0].as_let().clone())
}
