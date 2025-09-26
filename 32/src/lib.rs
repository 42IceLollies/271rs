#[macro_export]
macro_rules! choice{
    ($e:expr, $f:expr, $g:expr) => {
    // run on each bit
        ($e & $f) ^ ((!$e) & $g)
    };
}


#[macro_export]
macro_rules! majority{
    ($x:expr, $y: expr, $z: expr) => {
        ($x & $y) ^ ($y & $z) ^ ($z & $x)
    };
}


#[macro_export]
macro_rules! rotright{
    ($bits:expr, $n: expr)=>{{
          //Codeblock!!
          //find length of bitstringy thingy
          let l = size_of_val(&$bits)*8;
          //see how many times it needs to be rotated (rotate times % length)
          let i = $n % l;
          //make temp string & put the bits that will be overwritten on the beginning of it

          //bitshift the original bits
          //or with the original string and return
          
          ($bits << i) | ($bits >> l - i)
        }};
}






#[macro_export]
macro_rules! max {
    ( $x:expr, $y:expr, $z: expr ) => {
        if $x > $y && $x > $z {$x}
        else if $y>$z {$y}
        else {$z}

    };
}
