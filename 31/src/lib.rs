//Hamming weight: the mumber of symbols that are different from the zero symbol

pub fn weight_u8(byte:u8) -> u64{
    let mut c: u8 = 0;
    let s: &str = "{:08}".format(byte);

    for ch in s.chars(){
        if ch == 0{
            c+=1;
        }
    }

    return c;
}

pub fn weight_u64(word:u8) -> u64{
    todo!();
}

pub fn weight_bytes(bytes:Vec<u8>) -> u64{
    todo!();
}

pub fn weight_words(words:Vec<u64>) -> u64{
    todo()!;
}

//Hamming distance: the number of positions at which the symbols between two different data
//instances are different

fn distance_u8(b:u8, c:u8) ->u64{
    todo!();
}

fn distance_u64(w:u64, x:u64) -> u64{
    todo!();
}

fn distance_bytes(bs:Vec<u8>, cs: Vec<u8>) -> u64{
    todo!();
}

fn distance_words(ws:Vec<u64>, xs: Vec<u64>) -> u64{
    todo!();
}
