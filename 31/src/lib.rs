//Hamming weight: the mumber of symbols that are different from the zero symbol


pub fn weight_u8(byte:u8) -> u64{
    //this one is implemented via formatted string
    //formats the byte as an &str, then loops through the characters
    
    let mut c: u64 = 0;
    //let s = format!("{:08b}", byte);

    //for b in s.chars(){
       // if b == '1'{
      //      c+=1;
      //  }
    //}
    //return c;


    //implementation using bitwise operators
    
    //compare byte with 0000001
    //right shift
    
    for i in 0..8{
        if (byte >> i & 00000001) == 1{
            c+=1;
        }
    }

    return c;

}


pub fn weight_u64(word:u64) -> u64{
    let mut c: u64 = 0;
    //converts to a vector of u8s with native endianness (whatever endianness the computer uses
    //let bytes: [u8; 8] = word.to_ne_bytes();

    for i in 0..64{
        if (word>>i & 001) == 1{

            c+=1;
        }
    }

    return c;
}

pub fn weight_bytes(bytes:Vec<u8>) -> u64{
   let mut c: u64 = 0;

    for byte in bytes{
        c += weight_u8(byte);
    }

    return c;
}

pub fn weight_words(words:Vec<u64>) -> u64{
    let mut c: u64 = 0;

    for word in words{
        c+= weight_u64(word);
    }

    return c;
}

//Hamming distance: the number of positions at which the symbols between two different data
//instances are different

pub fn distance_u8(a:u8, b:u8) ->u64{
    //byte representing the bits that are the same between b and c
    let comparison: u8 = a^b;
    return weight_u8(comparison);

}

pub fn distance_u64(w:u64, x:u64) -> u64{
    let  comparison: u64 = w^x;
    return weight_u64(comparison);
}

pub fn distance_bytes(bs:Vec<u8>, cs: Vec<u8>) -> u64{
    let mut c: u64 = 0;
    if bs.len() != cs.len(){
        panic!("Byte vectors of differing lengths");
    }
    for i in 0..bs.len(){
        c+= distance_u8(bs[i], cs[i]);
    }

    return c;
}

pub fn distance_words(ws:Vec<u64>, xs: Vec<u64>) -> u64{
    let mut c: u64 = 0;
    if ws.len() != xs.len(){
        panic!("Word vectors of differing lengths");
    }
    for i in 0..ws.len(){
        c+= distance_u64(ws[i], xs[i]);
    }

    return c;
}

