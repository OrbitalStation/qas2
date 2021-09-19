use super::Code;
use crate::token::Token;

macro_rules! fundamental {
    ('full: $($c:literal $rust:literal),* 'alias: $($alias:literal $to:literal),*) => {
        fn all_types() {
            $(Self::add_full($c, $rust);)*
            $(Self::add_alias($alias, Self::c2id($to));)*
        }

        pub fn is_fundamental(ty: TypeID) -> bool {
            ty < (fundamental!{ @count $($c)* $($alias)* })
        }
    };

    (@count $t:literal) => {
        1
    };

    (@count $t:literal $($tt:literal)*) => {
        fundamental!{ @count $t } + fundamental!{ @count $($tt)+ }
    };
}

pub enum TypeType {
    Full { rust_name: String },

    Alias { id: TypeID },
}

pub struct Type {
    pub c_name: String,
    pub data: TypeType,
}

pub struct TypeConvert {
    pub from: TypeID,
    pub to: TypeID,
    pub convertor: TypeConvertFn,
}

pub type TypeID = usize;
pub type TypeConvertFn = fn(TypeID, &String) -> String;

static mut CONVERTS: Vec<TypeConvert> = Vec::new();
static mut TYPES: Vec<Type> = Vec::new();

impl Type {
    pub fn rust_name(&self) -> &String {
        let mut cur = self;
        while let TypeType::Alias { id } = cur.data {
            cur = &Self::types()[id]
        }
        if let TypeType::Full { ref rust_name } = cur.data {
            rust_name
        } else {
            unreachable!()
        }
    }

    pub fn true_c_name(&self) -> &String {
        let mut cur = self;
        while let TypeType::Alias { id } = cur.data {
            cur = &Self::types()[id];
        }
        &cur.c_name
    }

    pub fn types() -> &'static mut Vec<Type> {
        unsafe { &mut TYPES }
    }

    pub fn converts() -> &'static mut Vec<TypeConvert> {
        unsafe { &mut CONVERTS }
    }

    fn add(c_name: String, data: TypeType) {
        assert!(
            Self::rustify(&c_name).is_none(),
            "Type `{}` already exists",
            c_name
        );
        Type::types().push(Type { c_name, data })
    }

    pub fn add_full<S1, S2>(c_name: S1, rust_name: S2)
    where
        S1: ToString,
        S2: ToString,
    {
        Self::add(
            c_name.to_string(),
            TypeType::Full {
                rust_name: rust_name.to_string(),
            },
        )
    }

    pub fn add_alias<S>(c_name: S, id: TypeID)
    where
        S: ToString,
    {
        Self::add(c_name.to_string(), TypeType::Alias { id })
    }

    pub fn rustify(s: &String) -> Option<&'static str> {
        for i in Self::types() {
            if i.c_name == s.as_str() {
                return Some(&i.rust_name());
            }
        }
        None
    }

    pub fn trueify(s: &String) -> &'static String {
        Self::types()[Self::c2id(s)].true_c_name()
    }

    pub fn c2id(c_name: &str) -> TypeID {
        let mut i = 0;
        while i < Self::types().len() {
            if c_name == Self::types()[i].c_name {
                return i;
            }
            i += 1
        }
        unreachable!()
    }

    pub fn id2rust(id: TypeID) -> &'static str {
        Self::rustify(&Self::types()[id].c_name).unwrap()
    }

    pub fn id2c(id: TypeID) -> &'static String {
        &Self::types()[id].c_name
    }

    pub fn rust2id(rust_name: &'static str) -> TypeID {
        let mut i = 0;
        while i < Self::types().len() {
            if rust_name == Self::types()[i].rust_name() {
                return i;
            }
            i += 1
        }
        unreachable!()
    }

    pub fn true_id(id: TypeID) -> TypeID {
        Self::c2id(Self::true_c_name(&Self::types()[id]))
    }

    pub fn convert(mut from: TypeID, mut to: TypeID, data: &String) -> Option<String> {
        from = Self::true_id(from);
        to = Self::true_id(to);

        if Self::is_fundamental(from)
            && Self::is_fundamental(to)
            && from != Self::c2id("void")
            && to != Self::c2id("void")
        {
            return Some(simple_convertor(to, data));
        }

        for i in Self::converts() {
            if i.from == from && i.to == to {
                return Some((i.convertor)(i.to, data));
            }
        }
        None
    }

    // pub fn add_convert(mut from: TypeID, mut to: TypeID, convertor: TypeConvertFn) {
    //     from = Self::true_id(from);
    //     to = Self::true_id(to);
    //
    //     for i in Self::converts() {
    //         if i.from == from && i.to == to { panic!("cannot redefine convert") }
    //     }
    //     Self::converts().push(TypeConvert {
    //         from,
    //         to,
    //         convertor
    //     })
    // }

    pub fn init() {
        let mut i;
        for (a, b) in &[
            ("signed", "char"),
            ("unsigned", "char"),
            ("signed", "short"),
            ("unsigned", "short"),
            ("signed", "int"),
            ("unsigned", "int"),
            ("signed", "long"),
            ("unsigned", "long"),
        ] {
            i = 0;
            while i + 1 < Code::code().len() {
                if Code::code()[i] == Token::Name(String::from(*a))
                    && Code::code()[i + 1] == Token::Name(String::from(*b))
                {
                    Code::code().remove(i + 1);
                    if let Token::Name(ref mut x) = Code::code()[i] {
                        x.push(' ');
                        x.push_str(*b)
                    }
                }
                i += 1
            }
        }

        for (a, b, to) in &[(Token::Ampersand, Token::Ampersand, Token::DoubleAmpersand)] {
            i = 0;
            while i + 1 < Code::code().len() {
                if Code::code()[i].eq(a) && Code::code()[i + 1].eq(b) {
                    Code::code().remove(i + 1);
                    Code::code()[i] = to.clone();
                }
                i += 1
            }
        }

        Self::all_types()
    }

    fundamental! {
        'full:
            "signed char"    "i8",
            "unsigned char"  "u8",
            "signed short"   "i16",
            "unsigned short" "u16",
            "signed int"     "i32",
            "unsigned int"   "u32",
            "signed long"    "i64",
            "unsigned long"  "u64",

            "_Bool"          "bool",

            "void"           "()"

        'alias:
            "char"     "unsigned char",
            "short"    "signed short",
            "int"      "signed int",
            "long"     "signed long",
            "signed"   "signed int",
            "unsigned" "unsigned int"
    }
}

fn simple_convertor(to: TypeID, data: &String) -> String {
    format!(
        "{} as {}",
        if data.find(' ').is_none() {
            data.to_string()
        } else {
            format!("({})", data)
        },
        Type::id2rust(to)
    )
}
