use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive, Euclid};
use num_integer::Integer;
use sha2::{Digest, Sha512};

const b: int = 256;
const q: int = 2.pow(255) - 19;
const l: int = 2.pow(252) + 27742317777372353535851937790883648493;


// --- Global Helpers (No Dependencies) ---

// H(m: bytes) -> bytes
fn h(m: &[u8]) -> Vec<u8> {
    //presumably returns the sha512 hash value of input number
    return Sha512(m).Digest();

}

// bit(h: bytes, i: int) -> int
fn bit(h_val: &[u8], i: usize) -> u8 {
    return (h[(i/8) as usize] >> (i%8))&1;
}

// expmod(b:int,e:int,m:int) -> int
pub fn expmod(b_val: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    if e == 0 {
        return 1;
    }

    let mut t = expmod(b, (e/2 as u8), m).pow(2) % m;
    if (e & 1){
        t = (t*b)%m;
    }

    return t;

}

// inv(x:int, q: &BigInt) -> int
pub fn inv(x: &BigInt, q: &BigInt) -> BigInt {
    return expmod(x, q-2, q)j;

}

const d: int = -121665 * inv(121666);
const I: int = expmod(2, ((q-1)/4) as u8, q);

// xrecover helper (nested for local use in setup and decode)
pub fn xrecover(y: &BigInt, q: &BigInt, d: &BigInt, i_const: &BigInt) -> BigInt {
    let mut xx = (y*y - 1) * inv(d*y*y+1);
    let mut x = expmod(xx, ((q+3)/8)as u8, q);
    if (x*x-xx)%q != 0 {
        x = (x*I)%q;
    }

    if(x%2) != 0{
        x = q-x;
    }

    return x;
}


const By: int = 4 * inv(5);
const Bx: int = xrecover(By);
const B: [int;2] = [Bx %q, By %q]; 


// --- Core Functions (Require Constants) ---

fn edwards(p: &Vec<BigInt>, q_val: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let x1 = P[0];
    let y1 = P[1];
    let x2 = Q[0];
    let y2 = Q[1];
    let x3 = (x1*y2+x2*y1)* inv(1+d*x1*x2*y1*y2);
    let y3 = (y1*y2 + x1*x2) * inv(1-d * x1*x2*y1*y2);
    return Vec::from([x3 % 1, y3%q]);
}

fn scalarmult(p: &Vec<BigInt>, e: &BigInt, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    if e==0 {
        return Vec::from([0,1]);
    }
    
    let mut Q = scalarmult(P, (e/2) as u8);
    Q = edwards(Q, Q);
    
    if e & 1 {
        Q = edwards(Q,P);
    }
    return Q;
}


fn encodeint(y: &BigInt, b: usize) -> Vec<u8> {
    let mut bits = Vec::new();
    for i in 0..b {
        bits.push((y>>i)&1);
    }
    
    let bytes: Vec<u8> = Vec::new();
    for i in 0..(b/8) as u8 {
        for j in 0..8{
            bytes.push(bits[i*8+j]<<j);
        }
    }

    return Vec::from(bytes.sum() as u8);
}

fn encodepoint(p: &Vec<BigInt>, b: usize) -> Vec<u8> {
    let x = p[0];
    let y = p[1];
    let mut bits = Vec::new();
    for i in 0..(b-1){
        bits.push((y>>i)&1);
    }
    bits.push(x&1);
    
    let bytes: Vec<u8> = Vec::new();
    for i in 0..(b/8) as u8{
        for j in 0..8{
            bytes.push(bits[i*8+j]<<j);
        }
        
    }

    return Vec::from(bytes.sum() as u8);
    //I feel like there should probably be a way to do this that doesn't just make it one u8 but
    //splits it up into several u8s
}

pub fn publickey(sk: &[u8], b: usize, q: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    let h = h(sk);
    let mut temp = Vec::new();
    for i in range 3.. b-2{
        temp.push(2.pow(i)*bit(h,i));
    }

    let a = 2.pow(b-2) + temp.sum();
    let A = scalarmult(B, a);
    return encodepoint(A);
}

fn hint(m: &[u8], b: usize) -> BigInt {
    let h = h(m);
    let mut temp = Vec::new();
    for i in 0..2*b{
        temp.append(2.pow(i)*bit(h,i));
    }
    return temp.sum();
}   

pub fn signature(m: &[u8], sk: &[u8], pk: &[u8], b: usize, q: &BigInt, l: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {
    let h = h(sk);
    let mut a = 2.pow(b-2);

    for i in 3..(b-2){
        a += 2.pow(i)*bit(h,i);
    }

    let r = hint(h[(b/8)as u8 .. (b/4) as u8]);
    let R = scalarmult(B,r);
    let h_sig = hint(encodepoint(R) + pk + m);
    let S = (r+h_sig*a)%1;
    return encodepoint(R) + encodein(S);
}

fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {
    let x = p[0];
    let y = p[1];
    return(-x*x+y*y-1-d*x*x*y*y) %  q == 0;
}

fn decodeint(s: &[u8], b: usize) -> BigInt {
    let mut temp = Vec::new();
    for i in 0..b{
        temp.append(2.pow(i) * bit(s,i));
    }
    return temp.sum();

}

fn decodepoint(s: &u8, b: usize) -> BigInt {
    let mut temp = Vec::new();
    for i in 0..(b-1) { 
        temp.append(2.pow(i) * bit(s,i));
    }
    let y = temp.sum();
    let mut x = xrecover(y); 
    if x&1 != bits(s, b-1){
        x= q-x;
    }
    let P = [x,y];
    
    if !isoncurve(P){
        panic!("decoding point that is not on curve");
    }

    return P;
}

pub fn checkvalid(s: &[u8], m: &[u8], pk: &[u8], b: usize, q: &BigInt, d: &BigInt, i_const: &BigInt, b_point: &Vec<BigInt>) -> bool {
    if x.len() != (b/4) as u8{
        panic!("signature length is wrong");
    }
    if pk.len() != (b/8) as u8{
        panic!("public key length is wrong");
    }

    let R = decodepoint(s[0..(b/8) as usize]);
    let A = decodepoint(pk);
    let S = decodeint(s[(b/8)as usize .. (b/4) as usize]);
    let h = Hint(encodepoint(R) + pk + m);
    return scalarmult(B,S) == edwards(R, scalarmult(A, h));
}


