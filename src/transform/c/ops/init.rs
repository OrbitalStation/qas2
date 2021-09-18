use super::{Op, Type, Fix, Order};

macro_rules! init {
    (@binary0 $symbol:tt ($ty:expr) $order:ident $priority:literal $op:expr) => {
        Op::add(Op {
            operands: vec![Type::c2id($ty), Type::c2id($ty)],
            ret: Type::c2id($ty),
            parts: vec![String::from(stringify!($symbol))],
            fix: Fix::Prefix,
            order: Order::$order,
            priority: $priority,
            make: $op
        })
    };

    (@binary1 $symbol:tt $($ty:literal)* $order:ident $priority:literal $op:expr) => {
        $(
            init!(@binary0 $symbol (concat!("signed ", $ty)) $order $priority $op);
            init!(@binary0 $symbol (concat!("unsigned ", $ty)) $order $priority $op);
        )*
    };

    (@binary2 $symbol:tt $order:ident $priority:literal $op:expr) => {
        init!(@binary1 $symbol "char" "short" "int" "long" $order $priority $op);
    };

    (@binary $symbol:tt $order:ident $priority:literal $op:expr) => {
        init!(@binary2 $symbol $order $priority $op);
    };

    (@binary $symbol:tt $order:ident $priority:literal $op:expr, $($ty:literal)*) => {
        $(init!(@binary0 $symbol ($ty) $order $priority $op);)*
    };

    (@single $symbol:tt $fix:ident $priority:literal $op:expr, $($ty:literal)*) => {
        $(Op::add(Op {
            operands: vec![Type::c2id($ty)],
            ret: Type::c2id($ty),
            parts: vec![String::from(stringify!($symbol))],
            fix: Fix::$fix,
            order: Order::Left,
            priority: $priority,
            make: $op
        });)*
    };
}

pub fn init() {
    init!(@single + Prefix 10 |ops| format!("{}",  ops[0].name), "signed char" "signed short" "signed int" "signed long" "unsigned char" "unsigned short" "unsigned int" "unsigned long");
    init!(@single - Prefix 10 |ops| format!("-{}", ops[0].name), "signed char" "signed short" "signed int" "signed long");

    init!(@binary * Left 9 |ops| format!("{} * {}", ops[0].name, ops[1].name));
    init!(@binary / Left 9 |ops| format!("{} / {}", ops[0].name, ops[1].name));

    init!(@binary + Left 4 |ops| format!("{} + {}", ops[0].name, ops[1].name));
    init!(@binary - Left 4 |ops| format!("{} - {}", ops[0].name, ops[1].name));

    init!(@binary && Left 3 |ops| format!("{} && {}", ops[0].name, ops[1].name), "_Bool");
}
