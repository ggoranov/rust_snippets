// This struct needs to keep a immuatable reference to i32 value
// During the lifetime of an instance of I32Wrapper we need to ensure that
// the reference it points at will also be 'alive' otherwise we can have
// a dangling pointer "use after free".
//
// Lifetime notation 'a forces this restiction that Borrow checker promises to follow
struct I32Wrapper<'a> {
    x: &'a i32,
}

impl<'a> I32Wrapper<'a> {
    fn set_value(&mut self, y: &'a i32) {
        self.x = y;
    }
} 

fn main() {
    {
        let b: i32 = 222; 
        let wrapper;                        // ---+ wrapper life start from here
        let c: i32 = 333; 

        {
            let a: i32 = 111;               // -+ a goes into scope
            wrapper = I32Wrapper { x: &b }  // Borrow checker error will occur if a or c are used
        }                                   // ---+ a go out of scope
        
        let wrapper_ref: &I32Wrapper  = &wrapper;
        print!("Result: {}", wrapper_ref.x);
    }

    {
        let b: i32 = 222; 
        let wrapper = I32Wrapper { x: &b};
        {
            let a: i32 = 111;               // -+ a goes into scope
            wrapper.set_value(&a);          // Borrow checker error will occur if a or c are used
                            // ^ borrowed value does not live long enough
        }                                   // ---+ a go out of scope
        
        print!("Result: {}", wrapper.x);
    }
}
