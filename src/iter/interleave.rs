use std::marker::PhantomData;
use std::fmt;

#[derive(Debug, Clone)]
enum InterleaveState {
    A,
    B,
    Finished
}

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Clone)]
pub struct InterleaveIters<V, IA: Iterator<Item=V>, IB: Iterator<Item=V>> {
    iter_a: IA,
    iter_b: IB,
    state: InterleaveState,
    _v_marker: PhantomData<V>,
}

impl<V, IA: Iterator<Item=V> + fmt::Debug, IB: Iterator<Item=V> + fmt::Debug> fmt::Debug for InterleaveIters<V, IA, IB> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(stringify!(InterleaveIters))
            .field("iter_a", &self.iter_a)
            .field("iter_b", &self.iter_b)
            .field("state", &self.state)
            .finish()
    }
}

impl<V, IA: Iterator<Item=V>, IB: Iterator<Item=V>> Iterator for InterleaveIters<V, IA, IB> {
    type Item = V;

    #[inline]
    fn next(&mut self) -> Option<V> {
        use self::InterleaveState::*;

        match self.state {
            A => {
                let next = self.iter_a.next();
                self.state = if next.is_none() { Finished } else { B };
                next
            },
            B => {
                let next = self.iter_b.next();
                self.state = if next.is_none() { Finished } else { A };
                next
            },
            Finished => None
        }
    }
}

pub trait Interleave<V, IB: Iterator<Item=V>>
    where Self: Iterator<Item=V> + Sized
{
    fn interleave(self, other: IB) -> InterleaveIters<V, Self, IB> {
        InterleaveIters {
            iter_a: self,
            iter_b: other,
            state: InterleaveState::A,
            _v_marker: PhantomData,
        }
    }
}

impl<V, IA: Iterator<Item=V>, IB: Iterator<Item=V>> Interleave<V, IB> for IA
{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interleave_same_len() {
        let a = vec!['a', 'b'];
        let b = vec!['1', '2'];
        let c: Vec<_> = a.into_iter().interleave(b.into_iter()).collect();

        let expected = vec!['a', '1', 'b', '2'];

        assert_eq!(c, expected);
    }

    #[test]
    fn interleave_a_longer() {
        let a = vec!['a', 'b', 'c'];
        let b = vec!['1', '2'];
        let c: Vec<_> = a.into_iter().interleave(b.into_iter()).collect();

        let expected = vec!['a', '1', 'b', '2', 'c'];

        assert_eq!(c, expected);
    }

    #[test]
    fn interleave_b_longer() {
        let a = vec!['a', 'b'];
        let b = vec!['1', '2', '3'];
        let c: Vec<_> = a.into_iter().interleave(b.into_iter()).collect();

        let expected = vec!['a', '1', 'b', '2'];

        assert_eq!(c, expected);
    }
}