pub(super) fn get_reading<'a>() -> [&'a str; 2] {
    let value = "1234";

    let unit = " g";

    [value, unit]
}