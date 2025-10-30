fn main() {
    println!("========== integers ==========\n");

    let bit8: i8 =      100;
    let bit16: i16 =    0x0fff;
    let bit32: i32 =    0x0fffffff;
    let bit64: i64 =    0x0fffffffffffffff;
    let bit128: i128 =  0x0fffffffffffffffffffffffffffffff;

    println!("8 bits signed:        {}", bit8);
    println!("16 bits signed:       {}", bit16);
    println!("32 bits signed:       {}", bit32);
    println!("64 bits signed:       {}", bit64);
    println!("128 bits signed:      {}\n", bit128);

    let bit8: u8 =      0xff;
    let bit16: u16 =    0xffff;
    let bit32: u32 =    0xffffffff;
    let bit64: u64 =    0xffffffffffffffff;
    let bit128: u128 =  0xffffffffffffffffffffffffffffffff;

    println!("8 bits unsigned:      {}", bit8);
    println!("16 bits unsigned:     {}", bit16);
    println!("32 bits unsigned:     {}", bit32);
    println!("64 bits unsigned:     {}", bit64);
    println!("128 bits unsigned:    {}\n", bit128);

    // we can also declare variables like this but previous approach is better
    let _some_num = 35u8;

    // can also add _ in numbers for verbosity
    let bit8: u8 =      0xff;
    let bit16: u16 =    0xff_ff;
    let bit32: u32 =    0xffff_ffff;
    let bit64: u64 =    0xffff_ffff_ffff_ffff;
    let bit128: u128 =  0xffff_ffff_ffff_ffff_ffff_ffff_ffff_ffff;

    println!("8 bits unsigned:      {}", bit8);
    println!("16 bits unsigned:     {}", bit16);
    println!("32 bits unsigned:     {}", bit32);
    println!("64 bits unsigned:     {}", bit64);
    println!("128 bits unsigned:    {}\n", bit128);

    // usize and isize
    let num1: usize = 32;
    let num2: isize = -32;
    println!("Unsigned size:        {}", num1);
    println!("Signed size:          {}", num2);
}
