// Convert a base64 encoding of bytes (represented as ASCII characters) to the
// corresponding index in the base64 alphabet
pub fn base64_to_index(v: &[char]) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    let mut val: u8;

    for ch in v {
        let c: char = ch.clone();

        if c >= 'A' && c <= 'Z' {
            // 'A' to 'Z' -> 00 to 25
            val = c as u8 - 65u8;
        } else if c >= 'a' && c <= 'z' {
            // 'a' to 'z' -> 26 to 51
            val = c as u8 - 71u8;
        } else if c >= '0' && c <= '9' {
            // '0' to '9' -> 52 to 61
            val = c as u8 + 4u8;
        } else if c == '+' {
            // '+' -> 62
            val = 62u8;
        } else if c == '/' {
            // '/' -> 63
            val = 63u8;
        } else if c == '=' {
            // '=' -> padding -> 127
            val = 127u8;
        } else {
            // Unexpected character
            panic!("Unexpected character {} found!", c);
        }

        ret.push(val);
    }

    ret
}

// Convert a vector of base64 indices (0 - 63) into bytes
// 4 indices <-> 3 bytes
pub fn index_to_octet(v: &[u8]) -> Vec<u8> {
    let mut ret: Vec<u8> = Vec::new();
    let mut temp: u8 = 0;
    let v_len = v.len();

    for i in 0..v_len {
        if i % 4 == 0 {
            // Fill leftmost 6 bits
            temp = v.get(i).unwrap() << 2;
        } else if i % 4 == 1 {
            // Fill rightmost 2 bits and push to ret
            temp = temp | (v.get(i).unwrap() >> 4);
            ret.push(temp);
            // Reset temp and fill leftmost 4 bits
            temp = v.get(i).unwrap() << 4;
        } else if i % 4 == 2 {
            // Fill rightmost 4 bits and push to ret
            temp = temp | (v.get(i).unwrap() >> 2);
            ret.push(temp);
            // Reset temp and fill leftmost 2 bits
            temp = v.get(i).unwrap() << 6;
        } else {
            // Fill rightmost 6 bits and push to ret
            temp = temp | v.get(i).unwrap();
            ret.push(temp);
        }
    }

    ret
}
