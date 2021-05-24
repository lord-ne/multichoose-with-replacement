pub mod multi_index {
    use streaming_iterator::StreamingIterator;

    pub struct MultiIndexSum {
        arr : Vec<u32>,
        pos : usize,
        state : i8,
    }

    impl MultiIndexSum {
        pub fn new(sum : u32, len : usize) -> MultiIndexSum {
            assert_ne!(len, 0, "MultiIndexSum of length 0 is not valid");
            assert_ne!(sum, 0, "MultiIndexSum of sum 0 is not valid");
            let mut s = MultiIndexSum {arr : Vec::new(), pos : 0, state : -1};
            s.arr.resize(len, 0);
            s.arr[0] = sum;
            return s;
        }

        pub fn reset(&mut self) {
            let sum : u32 = self.arr.iter().sum();
            let len = self.arr.len();
            self.arr.clear();
            self.arr.resize(len, 0);
            self.arr[0] = sum;
            self.state = -1;
        }
    }

    impl StreamingIterator for MultiIndexSum {
        type Item = [u32];

        fn get(&self) -> Option<&Self::Item> {
            if self.state == 1 {
                return None;
            }
            return Some(self.arr.as_slice());
        }

        fn advance(&mut self) {
            // If we're done iterating, don't do anything
            if self.state == 1 {
                return;
            }
            // If we haven't started iterating, don't advance yet
            if self.state == -1 {
                self.state = 0;
                return;
            }

            // Subtract one from the last nonzero value, and add it to the next value
            if self.pos < self.arr.len() - 1 {
                self.arr[self.pos] -= 1;
                self.pos += 1;
                self.arr[self.pos] += 1;
            }
            // If the last nonzero value is the last value, use the one before that, and add the last one
            else {
                if let Some(found) = self.arr.iter().rev().skip(1).position(|x| *x != 0) {
                    let prev_pos = (self.arr.len() - 1) - (found + 1);
                    let new_val = self.arr[self.pos] + 1;
                    self.arr[prev_pos] -= 1;
                    self.arr[self.pos] = 0;
                    self.arr[prev_pos + 1] += new_val;
                    self.pos = prev_pos + 1;
                } 
                // Only the last value is nonzero, so we are done
                else {
                    self.state = 1;
                }
                
            }
        }
    }
}