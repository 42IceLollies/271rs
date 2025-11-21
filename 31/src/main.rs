//Hamming


//Bits notes:
//-----------------------------------------------------------------
//Bits/bytes vs types
//bytes - function- produces a rust itorator that produces u8 values
//Bytes - data structure 
//as_bytes - function that produces an array of u8s
//
//presumably each u8 represents 8 bits- can be thought of in decimal/printed out in decimal but its
//really bits
//to print in bit form use {:b} format code
// using {:08b} to render 8 zeroes before the byte - I assume this is so that the leading zeroes
// are included
//
//for bitwise operators - imagine an array of booleans and looping that operator over all of them
//exculsive or is equivalent to !=
//
//its more common to lead with the smaller number (big endianness?
//can compare bytes directly (x^y) -> returns a byte of this for each bit
//can turn bytes into string representation and process as a string

//package name is hamming 

fn main() {
    hamming::weight_u8(01010101);
}


