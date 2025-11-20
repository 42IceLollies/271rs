
type Heap<T> = Vec<T>;

fn heapify<T>(mut h: Heap<T>, mut i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    //test if newly input value is greater than its parent (at index floor(i/2)), if it is swap it,
    //do it again until it has no parent or it its parent is larger
    
    let mut half_i: usize = (i+1)/2;
    while i>0 && gt(&h[i],&h[half_i-1]){
        h.swap(i, half_i-1);

        i = half_i-1;
        half_i = (i+1)/2;
    }

    return h;
                  
}

fn reheapify<T>(mut h: Heap<T>, mut i: usize, gt: fn(&T, &T) -> bool) -> Vec<T> {
    //organize the items at each depth laterally
    //branches at depth = 2^i for i being a depth iterator
    
    //could we literally just compare it to the indeces right next to it? I think so because its
    //already organized so that this should be easy enough

    while i>0 && gt(&h[i], &h[i-1]){
        h.swap(i, i-1);
        i-=1;
    }

    return h;

}

fn vec_to_heap<T>(mut xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Heap<T> {
    for i in 0..xs.len(){
       xs = heapify(xs, i, gt);
    }

    return xs;
}   

fn heap_to_vec<T>(mut h: Heap<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    for i in 0..h.len(){
        h = reheapify(h, i, gt);
    
    }

    return h;
        
}

fn hsort<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    return heap_to_vec(vec_to_heap(xs, gt), gt);
}

fn main() {
    let xs: Vec<u64> = vec![2, 4, 6, 8, 5, 3, 7];
    fn f(x: &u64, y: &u64) -> bool {
        return x > y;
    }
    fn lt(x:&u64, y:&u64) -> bool{
        return x<y;
    }
    dbg!(&xs);
    let sorted: Vec<u64> = hsort(xs, f);
    //let sorted: Vec<u64> = vec_to_heap(xs, f);
    dbg!(&sorted);
    let resorted: Vec<u64> = hsort(sorted, lt);
    dbg!(&resorted);
    return;
}
