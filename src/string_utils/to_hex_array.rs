pub fn to_hex_array_fn(hex_string: &str) -> bool {
    // 检查长度合法
    if hex_string.len() % 2 != 0 {
        println!("ERROR : String len must be an even number!");
        return false;
    }
    // 检查字符合法
    for item in hex_string.chars() {
        if !item.is_ascii_hexdigit() {
            println!("ERROR : All char must be an hex digit!");
            return false;
        }
    }
    // 打印转换后的字符串
    print!("{{");
    for (i, item) in hex_string.chars().enumerate() {
        print!("{}", item);
        if i % 2 == 1 && (i + 1) != hex_string.len() {
            print!(",")
        }
    }
    print!("}}");
    return true;
}

#[test]
fn test() {
    let hex_string1 = "1234567890";
    let hex_string2 = "123456789A";
    let hex_string3 = "123456789R";
    let hex_string4 = "12345678901";
    assert_eq!(to_hex_array_fn(hex_string1), true);
    assert_eq!(to_hex_array_fn(hex_string2), true);
    assert_eq!(to_hex_array_fn(hex_string3), false);
    assert_eq!(to_hex_array_fn(hex_string4), false);
}
