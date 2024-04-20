pub fn bytes_to_int(v: &[u8]) -> i32 {
    if v.len() != 4 {
        panic!("v must contain 4 bytes");
    }

    let mut res: i32 = 0;
    for i in 0..v.len() {
        let curr: i32 = *v.get(i).unwrap() as i32;
        res = res | (curr << (8 * i));
    }

    res
}

pub fn bytes_to_unsigned_int(v: &[u8]) -> u32 {
    if v.len() != 4 {
        panic!("v must contain 4 bytes");
    }

    let mut res: u32 = 0;
    for i in 0..v.len() {
        let curr: u32 = *v.get(i).unwrap() as u32;
        res = res | (curr << (8 * i));
    }

    res
}

pub fn bytes_to_short(v: &[u8]) -> i16 {
    if v.len() != 2 {
        panic!("v must contain 2 bytes");
    }

    let mut res: i16 = 0;
    for i in 0..v.len() {
        let curr: i16 = *v.get(i).unwrap() as i16;
        res = res | (curr << (8 * i));
    }

    res
}

// TODO: implement bytes_to_float and bytes_to_double
#[allow(unused)]
pub fn bytes_to_float(v: &[u8]) -> f32 {
    if v.len() != 4 {
        panic!("v must contain 4 bytes");
    }

    // Split bytes into sign, exponent, mantissa
    let sign: i8 = (v.get(0).unwrap() >> 7).try_into().unwrap();

    let exponent: u8 = (v.get(0).unwrap() << 1) | (v.get(1).unwrap() >> 7);
    let exponent_as_power_of_2: u8 = (exponent - 127);

    let mantissa: u32 = ((v.get(0).unwrap() << 1) as u32) << 15
        | ((*v.get(2).unwrap() as u32) << 8)
        | *v.get(3).unwrap() as u32;

    // Build final float
    let mut res: f32 = sign.try_into().unwrap();

    res
}
