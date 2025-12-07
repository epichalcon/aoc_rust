use std::collections::HashSet;
use std::fmt::Debug;
use std::{collections::HashMap, hash::Hash};

use std::cmp::min;

#[derive(Debug, Clone)]
pub struct MultiSet<T: Eq + Hash> {
    counts: HashMap<T, usize>,
    total_elements: usize,
}

impl<T: Eq + Hash> MultiSet<T> {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
            total_elements: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.total_elements == 0
    }

    pub fn len(&self) -> usize {
        self.total_elements
    }

    pub fn distinct_elements(&self) -> usize {
        self.counts.len()
    }

    pub fn insert(&mut self, elem: T, times: usize) {
        if times == 0 {
            return;
        }
        if let Some(count) = self.counts.get(&elem) {
            self.counts.insert(elem, count + times);
        } else {
            self.counts.insert(elem, times);
        }
        self.total_elements += times;
    }

    pub fn remove(&mut self, elem: &T, times: usize) {
        if let Some(count) = self.counts.get_mut(elem) {
            if times >= *count {
                self.total_elements -= *count;
                self.counts.remove(elem);
            } else {
                *count -= times;
                self.total_elements -= times;
            }
        }
    }

    pub fn contains(&self, elem: &T) -> bool {
        self.counts.contains_key(elem)
    }

    pub fn count(&self, elem: &T) -> usize {
        if let Some(count) = self.counts.get(elem) {
            *count
        } else {
            0
        }
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I, count: usize) {
        for i in iter{
            self.insert(i, count);
        }
    }
}

impl<T: Eq + Hash + Clone> MultiSet<T> {
    pub fn intersection(&self, other: &Self) -> Self {
        self.iter().fold(Self::default(), |mut acc, (elem, count)| {
            if let Some(&other_count) = other.counts.get(elem) {
                acc.insert(elem.clone(), min(count, other_count));
            }
            acc
        })
    }

    pub fn union(&self, other: &Self) -> Self {
        other.iter().fold(self.clone(), |mut acc, (elem, count)| {
            acc.insert(elem.clone(), count);
            acc
        })
    }

    pub fn difference(&self, other: &Self) -> Self {
        other.iter().fold(self.clone(), |mut acc, (elem, count)| {
            acc.remove(elem, count);
            acc
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, usize)> {
        self.counts.iter().map(|(k, &v)| (k, v))
    }
}

impl<T: Eq + Hash> Default for MultiSet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a MultiSet<T> {
    type Item = (&'a T, &'a usize);
    type IntoIter = std::collections::hash_map::Iter<'a, T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.counts.iter()
    }
}

impl<T: Eq + Hash> FromIterator<T> for MultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut multiset = MultiSet::new();
        for i in iter {
            multiset.insert(i, 1);
        }
        multiset
    }
}

impl<T: Eq + Hash> FromIterator<(T, usize)> for MultiSet<T> {
    fn from_iter<I: IntoIterator<Item = (T, usize)>>(iter: I) -> Self {
        let mut multiset = MultiSet::new();
        for (i, times) in iter {
            multiset.insert(i, times);
        }
        multiset
    }
}

impl <T: Eq + Hash + Copy> From<HashSet<T>> for MultiSet<T> {
    fn from(value: HashSet<T>) -> Self {
        value.iter().copied().collect()
    }
}

impl <T: Eq + Hash + Copy> From<&HashSet<T>> for MultiSet<T> {
    fn from(value: &HashSet<T>) -> Self {
        value.iter().copied().collect()
    }
}

use std::ops::{Add, Sub, BitAnd, BitOr};

impl<T: Eq + Hash + Clone> Add for MultiSet<T> {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        for (elem, &count) in rhs.counts.iter() {
            result.insert(elem.clone(), count);
        }
        result
    }
}

impl<T: Eq + Hash + Clone> Add<&Self> for MultiSet<T> {
    type Output = Self;
    
    fn add(self, rhs: &Self) -> Self::Output {
        let mut result = self;
        for (elem, &count) in rhs.counts.iter() {
            result.insert(elem.clone(), count);
        }
        result
    }
}

impl<T: Eq + Hash + Clone> Sub for MultiSet<T> {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(&rhs)
    }
}

impl<T: Eq + Hash + Clone> Sub<&Self> for MultiSet<T> {
    type Output = Self;
    
    fn sub(self, rhs: &Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl<T: Eq + Hash + Clone> BitAnd for MultiSet<T> {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(&rhs)
    }
}

impl<T: Eq + Hash + Clone> BitAnd<&Self> for MultiSet<T> {
    type Output = Self;
    
    fn bitand(self, rhs: &Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<T: Eq + Hash + Clone> BitOr for MultiSet<T> {
    type Output = Self;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(&rhs)
    }
}

impl<T: Eq + Hash + Clone> BitOr<&Self> for MultiSet<T> {
    type Output = Self;
    
    fn bitor(self, rhs: &Self) -> Self::Output {
        self.union(rhs)
    }
}

// Also implement for references
impl<T: Eq + Hash + Clone> Add for &MultiSet<T> {
    type Output = MultiSet<T>;
    
    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        for (elem, &count) in rhs.counts.iter() {
                result.insert(elem.clone(), count);
        }
        result
    }
}

impl<T: Eq + Hash + Clone> Sub for &MultiSet<T> {
    type Output = MultiSet<T>;
    
    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl<T: Eq + Hash + Clone> BitAnd for &MultiSet<T> {
    type Output = MultiSet<T>;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<T: Eq + Hash + Clone> BitOr for &MultiSet<T> {
    type Output = MultiSet<T>;
    
    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_multiset() {
        let ms: MultiSet<i32> = MultiSet::new();
        assert!(ms.is_empty());
        assert_eq!(ms.len(), 0);
        assert_eq!(ms.distinct_elements(), 0);
    }

    #[test]
    fn test_insert_and_contains() {
        let mut ms = MultiSet::new();
        ms.insert("apple", 3);

        assert!(ms.contains(&"apple"));
        assert!(!ms.contains(&"banana"));
        assert_eq!(ms.len(), 3);
        assert_eq!(ms.distinct_elements(), 1);
    }

    #[test]
    fn test_insert_multiple_times() {
        let mut ms = MultiSet::new();
        ms.insert(42, 5);
        ms.insert(42, 3); // Should add to existing count

        assert_eq!(ms.len(), 8);
        assert_eq!(ms.distinct_elements(), 1);
    }

    #[test]
    fn test_remove_exact_count() {
        let mut ms = MultiSet::new();
        ms.insert("test", 5);
        ms.remove(&"test", 2);

        assert_eq!(ms.len(), 3);
        assert!(ms.contains(&"test"));
    }

    #[test]
    fn test_remove_more_than_exists() {
        let mut ms = MultiSet::new();
        ms.insert("test", 3);
        ms.remove(&"test", 5); // Should remove all 3

        assert!(!ms.contains(&"test"));
        assert_eq!(ms.len(), 0);
        assert!(ms.is_empty());
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut ms = MultiSet::new();
        ms.insert("a", 2);
        ms.remove(&"b", 1); // Should do nothing

        assert_eq!(ms.len(), 2);
        assert!(!ms.contains(&"b"));
    }

    #[test]
    fn test_intersection_basic() {
        let mut ms1 = MultiSet::new();
        ms1.insert(1, 3);
        ms1.insert(2, 2);

        let mut ms2 = MultiSet::new();
        ms2.insert(1, 2);
        ms2.insert(3, 4);

        let intersection = ms1.intersection(&ms2);

        assert_eq!(intersection.len(), 2); // Only element 1, min(3,2) = 2
        assert_eq!(intersection.distinct_elements(), 1);
        assert!(intersection.contains(&1));
        assert!(!intersection.contains(&2));
        assert!(!intersection.contains(&3));
    }

    #[test]
    fn test_intersection_empty() {
        let mut ms1 = MultiSet::new();
        ms1.insert("a", 2);

        let ms2 = MultiSet::new();

        let intersection = ms1.intersection(&ms2);

        assert!(intersection.is_empty());
        assert_eq!(intersection.len(), 0);
    }

    #[test]
    fn test_union_basic() {
        let mut ms1 = MultiSet::new();
        ms1.insert(1, 2);
        ms1.insert(2, 1);

        let mut ms2 = MultiSet::new();
        ms2.insert(1, 3);
        ms2.insert(3, 2);

        let union = ms1.union(&ms2);

        assert_eq!(union.len(), 8); 
        assert_eq!(union.distinct_elements(), 3);
        assert_eq!(union.count(&1), 5);
        assert_eq!(union.count(&2), 1);
        assert_eq!(union.count(&3), 2);
    }

    #[test]
    fn test_difference_basic() {
        let mut ms1 = MultiSet::new();
        ms1.insert("a", 5);
        ms1.insert("b", 3);
        ms1.insert("c", 2);

        let mut ms2 = MultiSet::new();
        ms2.insert("a", 2);
        ms2.insert("b", 3);
        ms2.insert("d", 4);

        let difference = ms1.difference(&ms2);

        // "a": 5-2=3, "b": 3-3=0, "c": 2-0=2, "d": 0-4=0 (not in ms1)
        assert_eq!(difference.len(), 5);
        assert_eq!(difference.distinct_elements(), 2);
        assert_eq!(difference.count(&"a"), 3);
        assert_eq!(difference.count(&"b"), 0);
        assert_eq!(difference.count(&"c"), 2);
        assert!(!difference.contains(&"b"));
    }

    #[test]
    fn test_difference_complete_removal() {
        let mut ms1 = MultiSet::new();
        ms1.insert("x", 3);

        let mut ms2 = MultiSet::new();
        ms2.insert("x", 5);

        let difference = ms1.difference(&ms2);

        assert!(difference.is_empty());
        assert_eq!(difference.len(), 0);
    }

    #[test]
    fn test_difference_with_zero() {
        let mut ms1 = MultiSet::new();
        ms1.insert("a", 2);

        let ms2 = MultiSet::new(); // Empty

        let difference = ms1.difference(&ms2);

        assert_eq!(difference.len(), 2);
        assert_eq!(difference.count(&"a"), 2);
    }

    #[test]
    fn test_complex_operations() {
        let mut ms = MultiSet::new();
        ms.insert(100, 10);
        assert_eq!(ms.len(), 10);

        ms.remove(&100, 3);
        assert_eq!(ms.len(), 7);

        ms.remove(&100, 10); // Remove more than exists
        assert!(ms.is_empty());

        ms.insert(200, 5);
        ms.insert(300, 5);
        assert_eq!(ms.distinct_elements(), 2);
    }

    #[test]
    fn test_edge_cases() {
        // Test with zero insert
        let mut ms = MultiSet::new();
        ms.insert("test", 0);
        assert!(ms.is_empty());
        assert!(!ms.contains(&"test"));

        // Test remove zero times
        ms.insert("test", 3);
        ms.remove(&"test", 0);
        assert_eq!(ms.len(), 3);
    }

    #[test]
    fn test_with_strings() {
        let mut ms = MultiSet::new();
        let s1 = String::from("hello");
        let s2 = String::from("world");

        ms.insert(s1.clone(), 2);
        ms.insert(s2.clone(), 1);

        assert!(ms.contains(&s1));
        assert!(ms.contains(&s2));
        assert_eq!(ms.len(), 3);
    }

    #[test]
    fn test_intersection_min_count() {
        let mut ms1 = MultiSet::new();
        ms1.insert(1, 5);

        let mut ms2 = MultiSet::new();
        ms2.insert(1, 2);

        let intersection = ms1.intersection(&ms2);
        assert_eq!(intersection.count(&1), 2); // min(5, 2)
    }

    #[test]
    fn test_union_max_count() {
        let mut ms1 = MultiSet::new();
        ms1.insert(1, 2);

        let mut ms2 = MultiSet::new();
        ms2.insert(1, 5);

        let union = ms1.union(&ms2);
        assert_eq!(union.count(&1), 7); 
    }

    #[test]
    fn test_difference_no_negative() {
        let mut ms1 = MultiSet::new();
        ms1.insert(1, 2);

        let mut ms2 = MultiSet::new();
        ms2.insert(1, 5);

        let difference = ms1.difference(&ms2);
        assert_eq!(difference.count(&1), 0); // 2 - 5 = 0 (not negative)
    }
}*/
