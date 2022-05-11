#![allow(unused)]
#![feature(decl_macro)]

fn main() {
    macro_rules! replace_expr_ {
        ($_t:tt $sub:expr) => {
            $sub
        };
    }
    macro replace_expr($_t:tt $sub:expr) {
        $sub
    }

    macro_rules! count_tts_ {
        () => { 0 };
        ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
        ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
    }
    macro count_tts {
        () => { 0 },
        ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 },
        ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 },
    }
}
