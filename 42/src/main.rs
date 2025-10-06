const H: [u64; 8] = [ 0x6a09e667f3bcc908,0xbb67ae8584caa73b, 0x3c6ef372fe94f82b,0xa54ff53a5f1d36f1,0x510e527fade682d1, 0x9b05688c2b3e6c1f, 0x1f83d9abfb41bd6b, 0x5be0cd19137e2179]; 

//FIX
const K: [u32; 64] = [0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
   0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
   0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
   0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
   0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
   0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
   0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
   0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2];


fn main() {
    
    //test value for hashing
    let mut bytes:Vec<u8> = Vec::new();
    bytes.append(&mut "Bleh".as_bytes().to_vec());

          
    
//DO THE PADDING THING
    //adds necesary bit padding to bit version of message to be hashed
    bytes.push(1);
    //K is a bytestring of the next multiple of 512 - (l+65) zeroes
    let bits = bytes.len()*8;
    bytes.append( &mut vec![0; (1024-(bits+129)%1024)]);

// Split into 1024 bit blocks
// chunks is the number of times 1024 fits into length of bits, then add remainder/remainder spaces
// (0 more if remainder is even, one more if not)
    
    let num_chunks: u32 = (bytes.len()*8/1024) as u32;
    let mut chunked: Vec<Vec<u8>> = Vec::new();
    let bytes_per_chunk: u32 = num_chunks/8; 
    for i in 0..num_chunks{
        chunked.append(Vec::new());
        for j in 0..bytes_per_chunk{
            chunked[i].append(bytes[j]);
        }
    }  



}

