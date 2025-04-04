// The value of a trait's associated constant is not known when lowering #[const_continue], and
// hence we can't determine which branch it should jump to.
#![allow(incomplete_features)]
#![feature(loop_match)]
#![crate_type = "lib"]

trait Foo {
    const Target: u8;

    fn test_u8(mut state: u8) -> &'static str {
        #[loop_match]
        loop {
            state = 'blk: {
                match state {
                    0 => {
                        #[const_continue]
                        break 'blk Self::Target;
                        //~^ ERROR could not determine the target branch for this `#[const_continue]`
                    }

                    1 => return "bar",
                    2 => return "baz",
                    _ => unreachable!(),
                }
            }
        }
    }
}
