use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub trait CollectMapVec : Iterator
{
    fn collect_map_vec_by<K, V, FA>(self, f: FA) -> HashMap<K, Vec<V>>
        where Self: Sized + Iterator<Item=V>,
            K: Hash + Eq,
            FA: Fn(&V) -> K
    {
        self.map(|v| (f(&v), v)).collect_map_vec()
    }

    fn collect_map_vec<K, V>(self) -> HashMap<K, Vec<V>>
        where Self: Sized + Iterator<Item=(K, V)>,
              K: Hash + Eq
    {
        let mut map = HashMap::<K, Vec<V>>::new();

        for (key, val) in self {
            let vec = map.entry(key).or_insert(vec![]);
            vec.push(val);
        }

        map
    }
}

impl<T: ?Sized> CollectMapVec for T
    where T: Iterator {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_by() {
        let ints = 1_u32..10;
        let odd_even = ints.collect_map_vec_by(|i| i%2 == 0);

        let mut expected = HashMap::new();
        expected.insert(true, vec![2,4,6,8]);
        expected.insert(false, vec![1,3,5,7,9]);

        assert_eq!(odd_even, expected);
    }

    #[test]
    fn group() {
        let ints = 1_u32..10;
        let odd_even = ints.map(|i| (i%2 == 0, i))
            .collect_map_vec();

        let mut expected = HashMap::new();
        expected.insert(true, vec![2,4,6,8]);
        expected.insert(false, vec![1,3,5,7,9]);

        assert_eq!(odd_even, expected);
    }
}