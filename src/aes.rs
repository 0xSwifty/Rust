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

pub fn sub_bytes(byte: u8) {
    let mut bit = 0;

    let mut new_byte = Vec::new();

    while bit < 8 {
        let bits = affine_transformation(bit, byte);
        new_byte.push(bits);
        bit += 1;
    }

    println!("{:?} ", new_byte);
}

pub fn affine_transformation(bit: usize, byte: u8)  -> usize {
    let mut bits = Vec::new();
    
    let c = 0x63;

    let padding = 4;
    for i in 0..4 {
        let a = (bit + padding + i) % 8;

        let binary = (a >> i) & 1;
        bits.push(binary);
    }

    bits.push((c >> bit) & 1);


    let mut xor = 0;

    for i in 0..4 {
        xor = bits[i] | bits[i + 1];
    }
    
    xor
}

pub fn xtimes(v: u8) -> u8 {
    let mut x = 0;
    if (v & 0b10000000) != 0 {
        x = (v << 1) ^ 0x1B;
    } else {
        x = v << 1;
    }
    println!("xTimes({{{:x}}}) = {{{:x}}}", v, x);
    v.reverse_bits()
}

/*

    Value: 0xA5
    in binary: 1010 0101

[
    b0  1
    b1  0
    b2  1
    b3  0
    b4  0
    b5  1
    b6  0
    b7  1
]

0x63
[ 0 1 1 0 0 0 1 1 ]

0xA5
[
    b0 = 1 0 1 0 1 0
    xor = 1
]
*/
