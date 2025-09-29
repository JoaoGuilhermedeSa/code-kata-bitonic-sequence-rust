fn main() {
    // solution(n, l, r);
    let solution = bitonicArray(5, 3, 10);
    println!("{:?}", solution);
}

fn bitonicArray(n: usize, l: usize, r: usize) -> Vec<i32> {
    println!("{}", n);
    println!("{}", l);
    println!("{}", r);

    let mut my_array_solution = Vec::with_capacity(n);

    //Step 1: If not possible
    if(n > (r-l) *2 +1){
        return vec![-1];
    }
    
    my_array_solution
}

//output 9 10 9 8 7 