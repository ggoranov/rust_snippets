/// Borrow checker restrictive rules

// Borrowing 1: Immutable references (a.k.a immutable borrow)
// - v1 here does not own the resource
//      but only borrows ownership for the
//      scope of the function
// -    A binding that borrows something does
//      NOT deallocate the resource when it goes out of scope
// -    Read-only access to a resource
// -    one or more immutable borrows to a resource a the same time
// Usage Example:
fn foo(_v1: &Vec<i32>, _v2: &Vec<i32>) {
    //read-only access

    // Error Example: v1.push(10);
    //
    // consider changing this to be a mutable reference
    // label: `v1` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

// Borrowing 2: Mutable reference (a.k.a  mutable borrow)
// - v1 here does not own the resource
//      but only borrows ownership for the
//      scope of the function
// - A binding that borrows something does
//      NOT deallocate the resource when it
//      goes out of scope
// - Data can be modified
// - Only one mutable reference allowed per a time
// Usage Example:
fn foo_2(v: &mut Vec<i32>) {
    v.push(10);

    if v.len() < 10 {
        // TODO ???
        foo_2(v);
    }
}

fn main() {
    let mut v1 = vec![1, 2, 3];
    let v2 = vec![3, 4, 5];

    foo(&v1, &v2);
    // Error Example: types differ in mutability
    // v2 is immutable whereas foo_2(v1 ) is mutable
    // foo_2(&v2);

    // Use of mutable reference
    // NB: Even when we pass mutable reference we need
    // mut keyword
    foo_2(&mut v1);

    foo_3();

    foo_4();

    /// Cannot use them more: already moved
    print!("{:?}", v1);
}

// Examples
// Restrictions when a mutable borrow exists
fn foo_3() {
    print!("\n ========= foo_3 ========= \n");
    let mut x = 5;

    // Until 'y' mutablly borrows 'x'
    // no other mutable borrows are possible
    {
        let y = &mut x; // y &mut borrow starts here
        *y += 1;

        // Error Example:
        // cannot borrow `x` as immutable because it is also borrowed as mutable
        // print!("{}", x);

        // Error Example:
        // cannot borrow `x` as immutable because it is also borrowed as mutable
        // let y2 = &x;

        // Error Example:
        // assignment to borrowed `x` occurs here
        // x = 7;

        // Error: second mutable borrow occurs here
        // let z = &mut x;
    } // -+ ... y &mut borrow and ends here

    // mutable borrow - y is out of scope. So now we can
    // borrow again x
    print!("{}\n", x);
    x = 7;
    let _z = &mut x;
}

fn foo_4() {
    print!("\n ========= foo_4 ========= \n");
    {
        // Edition: 2018. A bit different with 2015
        let y: &i32;
        {
            let x = 5;
            y = &x;
        } //  - `x` dropped here while still borrowed

        // println!("{}", y);
        //             ^ borrow of possibly uninitialized variable: `y`
    }
}
