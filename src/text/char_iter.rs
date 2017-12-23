use std::ops::{Range, RangeInclusive};
use std::collections::range::RangeArgument;
use std::convert::TryFrom;

pub trait RangeMarker {}

impl<T> RangeMarker for Range<T> {}
impl<T> RangeMarker for RangeInclusive<T> {}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CharIter {
    start: u32,
    end: u32,
}

impl CharIter {
    pub fn new<R: RangeMarker + RangeArgument<char>>(r: R) -> CharIter {
        use std::collections::Bound::*;

        let start = match r.start() {
            Included(&s) => s as u32,
            Excluded(&s) => CharIter::prev_char(s) as u32,
            Unbounded => unreachable!(),
        };

        let end = match r.end() {
            Included(&s) => s as u32,
            Excluded(&s) => CharIter::prev_char(s) as u32,
            Unbounded => unreachable!(),
        };

        CharIter {
            start: start,
            end: end
        }
    }

    fn prev_char(c: char) -> char {
        let next = (0..c as u32).rev()
            .filter_map(|c| char::try_from(c).ok())
            .next();

        next.unwrap()
    }

    fn next_char(c: char) -> char {
        let next = (c as u32+1..)
            .filter_map(|c| char::try_from(c).ok())
            .next();

        next.unwrap()
    }
}

impl Iterator for CharIter {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            return None;
        }

        let cur = char::try_from(self.start).unwrap();

        self.start = CharIter::next_char(cur) as u32;

        Some(cur)
    }
}

impl DoubleEndedIterator for CharIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end < self.start {
            return None;
        }

        let cur = char::try_from(self.end).unwrap();

        self.end = CharIter::prev_char(cur) as u32;

        Some(cur)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prev_char() {
        assert_eq!('A', CharIter::prev_char('B'))
    }

    #[test]
    fn next_char() {
        assert_eq!('B', CharIter::next_char('A'))
    }

    #[test]
    fn construct_exclusive() {
        let expected = CharIter {
            start: 'A' as u32,
            end: 'D' as u32,
        };

        let actual = CharIter::new('A'..'E');

        assert_eq!(expected, actual);
    }

    #[test]
    fn construct_inclusive() {
        let expected = CharIter {
            start: 'A' as u32,
            end: 'E' as u32,
        };

        let actual = CharIter::new('A'..='E');

        assert_eq!(expected, actual);
    }

    #[test]
    fn a_to_e() {
        let expected = vec!['A', 'B', 'C', 'D', 'E'];
        let actual: Vec<_> = CharIter::new('A'..='E').collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn e_to_a() {
        let expected = vec!['E', 'D', 'C', 'B', 'A'];
        let actual: Vec<_> = CharIter::new('A'..='E').rev().collect();

        assert_eq!(expected, actual);
    }
}