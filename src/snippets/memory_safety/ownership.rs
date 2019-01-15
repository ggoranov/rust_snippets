// Move ownership is used so that only v1 (data_pointer, length value and capacity value)
// are copied to the caller stack frame
//but the allocated 1MB heap memory is not copied.
#[allow(dead_code)]
fn allocate_vec_heap_memory() -> (Vec<u8>, String) {
    let mut v1 = vec![0u8; 1024 * 1024];
    let addr_as_str = format!("{:?}", v1.as_mut_ptr());
    (v1, addr_as_str)
}

fn read_vec_heap_memory() {
    {
        let (mut v2, addr_as_str) = allocate_vec_heap_memory(); // v2 gets the ownership of the allocated heap
                                                                // and the responsibility to free memory when out of scope
        let moved_addr_as_str = format!("{:?}", v2.as_mut_ptr());
        assert_eq!(moved_addr_as_str, addr_as_str);
    } // v2 is getting out of scope as a result allocated heap
      // is freed and no memory leak occurs.
}
#[test]
fn test_dynamic_memory() {
    // Run with `valgrind --leak-check=full  ./main` to confirm lack of leaks.
    read_vec_heap_memory();
}

fn main() {
    sample_1();

    sample_2();
}

fn sample_1() {
    read_vec_heap_memory();
    print!("read_vec_heap_memory done")
}

fn sample_2() {
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

    // Cannot use them more: already moved
    //  print!("{:?}", v1);}
}
