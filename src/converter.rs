pub fn convert(from: &str, to: &str, val: &str) -> Result<String, String> {
    let int_value = match from {
        "b" => from_bin_to_int(val)?,
        "o" => from_oct_to_int(val)?,
        "d" => from_dec_to_int(val)?,
        "h" => from_hex_to_int(val)?,
        _ => return Err(format!("Not a valid system: {}", from))
    };

    match to {
        "b" => Ok(from_int_to_bin(int_value)),
        "o" => Ok(from_int_to_oct(int_value)),
        "d" => Ok(from_int_to_dec(int_value)),
        "h" => Ok(from_int_to_hex(int_value)),
        _ => Err(format!("Not a valid system: {}", from))
    }
}

//to int

pub fn from_dec_to_int(val: &str) -> Result<u128, String> {
    let mut dec = 0;

    for (i, c) in val.chars().rev().enumerate() {

        if dec > (u128::MAX / 10) {
            return Err(format!("Number too big: {}", val));
        }

        let exp = u128::pow(10, i as u32);
        dec += exp * match c.to_ascii_lowercase() {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => return Err(format!("Not a valid decimal number: {} at char {}: {}", val, i, c))
        };
    }
    return Ok(dec);
}

pub fn from_oct_to_int(val: &str) -> Result<u128, String> {
    let mut dec = 0;

    for (i, c) in val.chars().rev().enumerate() {

        if dec > (u128::MAX / 8) {
            return Err(format!("Number too big: {}", val));
        }

        let exp = u128::pow(8, i as u32);
        dec += exp * match c.to_ascii_lowercase() {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            _ => return Err(format!("Not a valid octal number: {} at char {}: {}", val, i, c))
        };
    }
    return Ok(dec);
}

pub fn from_hex_to_int(val: &str) -> Result<u128, String> {
    let mut dec = 0;

    for (i, c) in val.chars().rev().enumerate() {

        if dec > (u128::MAX / 16) {
            return Err(format!("Number too big: {}", val));
        }

        let exp = u128::pow(16, i as u32);
        dec += exp * match c.to_ascii_lowercase() {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            _ => return Err(format!("Not a valid hex number: {} at char {}: {}", val, i, c))
        };
    }
    return Ok(dec);
}

pub fn from_bin_to_int(val: &str) -> Result<u128, String> {
    let mut dec = 0;

    for (i, c) in val.chars().rev().enumerate() {

        if dec > (u128::MAX / 2) {
            return Err(format!("Number too big: {}", val));
        }

        let exp = u128::pow(2, i as u32);
        if c == '1' {
            dec += exp;
        } else if c == '0' {} else {
            return Err(format!("Not a valid binary number: {} at char {}: {}", val, i, c));
        }
    }

    return Ok(dec);
}


//FROM INT

pub fn from_int_to_oct(mut val: u128) -> String {
    let mut s = String::new();
    if val == 0 {
        s += "0";
    }
    while val > 0 {
        let rest = val % 8;
        s += &rest.to_string();
        val = val / 8;
    }

    return s.chars().rev().collect();
}

pub fn from_int_to_hex(mut val: u128) -> String {
    let mut s = String::new();
    if val == 0 {
        s += "0";
    }
    while val > 0 {
        let rest = val % 16;
        let to_string = rest.to_string();

        s += match rest {
            10 => "A",
            11 => "B",
            12 => "C",
            13 => "D",
            14 => "E",
            15 => "F",
            _ => &to_string
        };
        val = val / 16;
    }

    return s.chars().rev().collect();
}

pub fn from_int_to_bin(mut val: u128) -> String {
    let mut s = String::new();
    if val == 0 {
        s += "0";
    }
    while val > 0 {
        let rest = val % 2;
        s += &rest.to_string();
        val = val / 2;
    }

    return s.chars().rev().collect();
}

pub fn from_int_to_dec(val: u128) -> String {
    val.to_string()
}