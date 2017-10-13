use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub trait GroupByKey<V>
    where Self: Iterator<Item=V> + Sized
{
    fn group_by_key<K, FA>(self, f: FA) -> HashMap<K, Vec<V>>
        where K: Hash + Eq,
            FA: Fn(&V) -> K
    {
        let mut map = HashMap::<K, Vec<V>>::new();

        for val in self {
            let key = f(&val);
            let vec = map.entry(key).or_insert(vec![]);
            vec.push(val);
        }

        map
    }
}

impl<V, I> GroupByKey<V> for I
    where Self: Iterator<Item=V> + Sized
{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group() {
        let ints = 1_u32..10;
        let odd_even = ints.group_by_key(|i| i%2 == 0);

        let mut expected = HashMap::new();
        expected.insert(true, vec![2,4,6,8]);
        expected.insert(false, vec![1,3,5,7,9]);

        assert_eq!(odd_even, expected);
    }
}