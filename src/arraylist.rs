
#[derive(Debug)]
std::Vec;
pub struct ArrayList<'a, T> {
    array: &'a mut [&'a mut T],
    len: usize,
}

impl<'a, T> Default for ArrayList<'a, T> {
    fn default() -> Self {
        ArrayList{
            array: [].as_mut(),
            len: 0, 
        }
    }
}

impl<'a, T> ArrayList<'a, T> {
    fn push(&mut self, item: &mut T) {
        if self.len  < self.array.len() {
            // no need to increase array size
            let next = self.len + 1;

            self.array[next] = item; 
            self.len = self.len + 1;
        }
        else {
            // have to increase array size
            let old_array = self.array;
            let mut new_array: &'a mut [&'a mut T; self.array.len() * 2];
            let mut i = 0;
            for item in old_array {
                new_array[i] = item; 
            }
            // increased size by double current size
            
            let next = self.len + 1;

            self.array[next] = item; 
            self.len = self.len + 1;
        }

    }

     


}