use std::io::Error;

pub fn string_test() -> Result<(String, String, String), Error> {
    // heap allocated strings
    let a = std::any::type_name::<String>();
    let b = std::any::type_name::<&String>();

    // stack allocated strings
    let c = std::any::type_name::<&str>();

    Ok((a.into(), b.into(), c.into()))
}