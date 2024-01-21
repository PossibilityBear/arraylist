
#[derive(Debug)]
pub struct ArrayList<'a, T> {
    array: &'a [T],
    len: usize,
}

impl<'a, T> Default for ArrayList<'a, T> {
    fn default() -> Self {
        ArrayList{
            array: &[],
            len: 0, 
        }
    }
}

