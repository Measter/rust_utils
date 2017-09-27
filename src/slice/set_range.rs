use std::collections::range::RangeArgument;
use std::slice::SliceIndex;
    
pub trait SetRange<R, T>
{
    fn set_range(&mut self, r: R, v: T);
}

impl<'a, R, T: Copy> SetRange<R, T> for &'a mut [T]
    where R: RangeArgument<usize> + Iterator<Item=usize> + SliceIndex<[T], Output=[T]>
{
    fn set_range(&mut self, r: R, v: T) {
        self[r].iter_mut().for_each(|i| *i = v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let mut vals = vec![0; 5];
        { 
            let mut vals = &mut vals[..];
            vals.set_range(1..3, 2);
        }

        assert_eq!(vals, vec![0,2,2,0,0]);
    }
}