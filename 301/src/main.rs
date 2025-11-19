
type Heap<T> = Vec<T>;

fn heapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    //test if newly input value is greater than its parent (at index floor(i/2)), if it is swap it,
    //do it again until it has no parent or it its parent is larger
    
    let mut half_i: usize = ((i+1)/2)-1;

    while i>0 && gt(&h[i],&h[half_i]){
        
        let temp = &mut h[half_i];
        h[half_i] = h[i];
        h[i] = *temp;

        i = half_i;
        half_i = ((i+1)/2)-1;
    }

    return h;
                  
}

fn reheapify<T>(mut h: Heap<T>, i: usize, gt: fn(&T, &T) -> bool) -> Heap<T> {
    todo!();
    //not really sure what this function is for :/
}

fn vec_to_heap<T>(mut xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Heap<T> {
    for i in 0..xs.len(){
       xs = heapify(xs, i, gt);
    }

    return xs;
}   

fn heap_to_vec<T>(mut h: Heap<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    todo!();
}

fn hsort<T>(xs: Vec<T>, gt: fn(&T, &T) -> bool) -> Vec<T> {
    return heap_to_vec(vec_to_heap(xs, gt), gt);
}

fn main() {
    let xs: Vec<u64> = vec![2, 4, 6, 8, 5, 3, 7];
    fn f(x: &u64, y: &u64) -> bool {
        return x > y;
    }
    dbg!(&xs);
    //let sorted: Vec<u64> = hsort(xs, f);
    let sorted: Vec<u64> = vec_to_heap(xs, f);
    dbg!(&sorted);
    return;
}
