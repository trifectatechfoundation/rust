//@ run-pass

#![feature(loop_match)]

fn main() {
    let mut state = 0;
    #[loop_match]
    'a: loop {
        state = 'blk: {
            match state {
                0 =>
                {
                    #[const_continue]
                    1
                }
                1 => {
                    if true {
                        #[const_continue]
                        break 'blk 2;
                    } else {
                        // No drops allowed at this point
                        #[const_continue]
                        break 'blk 3;
                    }
                }
                _ => break 'a,
            }
        }
    }
}
