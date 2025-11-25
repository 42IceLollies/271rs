pub struct F16 {
    bits: u16,
    sign: u16,
    exp: u16,
    mantissa:u16,
}

pub fn f16(bits:u16, sign:u16, exp:u16, mantis:u16)-> F16 {
    F16{
        bits,
        sign, 
        exp,
        mantis
    }
}

pub fn i32_to_f16(n:i32) -> F16{
    
    F16{
        //bits:
        //just divide the int by two to find its bits and mantissa??? 
        //should probably review the lecture for this :/
    }
}

fn print_f16(x: F16){
    print!("{}{}.{}*2^{}", x.sign, x.bits, x.mantissa,x.exponent );
}

fn println_f16(x:f16){
    println!("{}{}.{}*2^{}", x.sign, x.bits, x.mantissa,x.exponent );

}
