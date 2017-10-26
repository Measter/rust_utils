pub trait SetRange<T>
{
    fn set(&mut self, v: T);
}

impl<'a, T: Copy> SetRange<T> for [T]
{
    fn set(&mut self, v: T) {
        self.iter_mut().for_each(|i| *i = v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range() {
        let mut vals = vec![0; 5];
        vals[1..3].set(2);

        assert_eq!(vals, vec![0,2,2,0,0]);
    }
}