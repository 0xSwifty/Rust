// AES-128
const NK: u8 = 4;
const NB: u8 = 4;
const NR: u8 = 10;

pub fn input() -> u8 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .unwrap();
    input.trim().parse().unwrap()
}

pub fn xtimes(v: u8) -> u8 {
    let mut x = 0;
    if (v & 0b10000000) != 0 {
        x = (v << 1) ^ 0x1B;
    } else {
        x = v << 1;
    }
    println!("xTimes({{{:x}}}) = {{{:x}}}", v, x);
    v
}
