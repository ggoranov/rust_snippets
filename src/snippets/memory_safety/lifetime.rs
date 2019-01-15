// This struct needs to keep an immuatable reference to i32 value
// During the lifetime of an instance of I32Wrapper we need to ensure that
// the reference it points at will also be 'alive' otherwise we can have
// a dangling pointer "use after free".
//
// Lifetime notation 'a forces this restiction that Borrow checker promises
// to check for us anytime I32Wrapper is used
struct I32Wrapper<'a> {
    x: &'a i32,
}

// An example where we force a restriction that any passed immutable reference must be
// alive as much as an instance of I32Wrapper
impl<'a> I32Wrapper<'a> {
    fn set_value(&mut self, y: &'a i32) {
        self.x = y;
    }
}

fn main() {
    // Example 1
    {
        let b: i32 = 222;
        let wrapper; // ---+ wrapper life start from here
        let c: i32 = 333;

        {
            let a: i32 = 111; // -+ a goes into scope
            wrapper = I32Wrapper { x: &b } // Borrow checker error will occur if a or c are used
        } // ---+ a go out of scope

        let wrapper_ref: &I32Wrapper = &wrapper;
        print!("Result: {}", wrapper_ref.x);
    }

    // Example 2
    {
        let b: i32 = 222;
        let wrapper = I32Wrapper { x: &b };
        {
            let a: i32 = 111; // -+ a goes into scope
            // wrapper.set_value(&a); // Borrow checker error will occur if a or c are used
                                   // ^ borrowed value does not live long enough
        } // ---+ a go out of scope

        print!("Result: {}", wrapper.x);
    }

    // Example 3
    {
        struct I32WrapperFields<'a> {
            field1: &'a i32,
            field2: &'a i32,
        }

        let x = 5;
        {
            let y = 10;
            let f = I32WrapperFields {
                field1: &x,
                field2: &y,
            };
        }
    }
}
