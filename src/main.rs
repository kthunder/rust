mod string_utils;

use string_utils::to_hex_array::to_hex_array_fn;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg1 = &args[1];
    let arg2 = &args[2];

    println!("{:#?}", args);

    if arg1.eq("tohex") {
        to_hex_array_fn(arg2);
    }
}
