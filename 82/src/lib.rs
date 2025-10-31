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
    //haven't implemented this one yet ---------------------
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
    let mut x = expmod(xx, ((q+3)/8)as u8), q);
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
    let bits = Vec::new();
    for i in 0..b {
        bits.push((y>>i)&1);
    }
    
    let bytes: Vec<u8> = Vec::new();
    for i in 0..8 {
        bytes.push(bits[i*8+j]<<j);
    }

    return Vec::from(bytes.sum() as u8);
}

fn encodepoint(p: &Vec<BigInt>, b: usize) -> Vec<u8> {
    
}

pub fn publickey(sk: &[u8], b: usize, q: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {

fn hint(m: &[u8], b: usize) -> BigInt {

pub fn signature(m: &[u8], sk: &[u8], pk: &[u8], b: usize, q: &BigInt, l: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {

fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {

fn decodeint(s: &[u8], b: usize) -> BigInt {

pub fn checkvalid(s: &[u8], m: &[u8], pk: &[u8], b: usize, q: &BigInt, d: &BigInt, i_const: &BigInt, b_point: &Vec<BigInt>) -> bool {
