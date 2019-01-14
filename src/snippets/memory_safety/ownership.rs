fn main() {
    fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
        (v1, v2, 21)
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![3, 4, 5];

    /*
    stack memory:
    v1
        heap_mem address
        capacity value
        length value


    heap memory
    v1
        1
        2
        3
    */

    foo(v1, v2);

    /// Cannot use them more: already moved
    ///  print!("{:?}", v1);
}
