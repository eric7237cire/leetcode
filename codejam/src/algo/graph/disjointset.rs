/*
 * Disjoint-set data structure - Library (Rust)
 *
 * Copyright (c) 2018 Project Nayuki. (MIT License)
 * https://www.nayuki.io/page/disjoint-set-data-structure
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 * - The above copyright notice and this permission notice shall be included in
 *   all copies or substantial portions of the Software.
 * - The Software is provided "as is", without warranty of any kind, express or
 *   implied, including but not limited to the warranties of merchantability,
 *   fitness for a particular purpose and noninfringement. In no event shall the
 *   authors or copyright holders be liable for any claim, damages or other
 *   liability, whether in an action of contract, tort or otherwise, arising from,
 *   out of or in connection with the Software or the use or other dealings in the
 *   Software.
 */

use std;

/*
 * Represents a set of disjoint sets. Also known as the union-find data structure.
 * Main operations are querying if two elements are in the same set, and merging two sets together.
 * Useful for testing graph connectivity, and is used in Kruskal's algorithm.
 */
#[derive(Clone)]
pub struct DisjointSet
{
    numberofsets: usize,

    nodes: Vec<DisjointSetNode>,
}

// Private helper structure.
#[derive(Clone, Copy)]
struct DisjointSetNode
{
    // The index of the parent element. An element is a representative iff its parent is itself.
    parent: usize,

    // Always in the range [0, floor(log2(NumberOfElements))]. Thus has a maximum value of 63.
    rank: i8,

    // Positive number if the element is a representative, otherwise zero.
    size: usize,
}

impl DisjointSet
{
    // Constructs a new set containing the given number of singleton sets.
    // For example, new DisjointSet(3) --> {{0}, {1}, {2}}.
    pub fn new(numelems: usize) -> Self
    {
        Self {
            numberofsets: numelems,
            nodes: (0..numelems)
                .map(|i| DisjointSetNode {
                    parent: i,
                    rank: 0,
                    size: 1,
                })
                .collect(),
        }
    }

    // Returns the number of elements among the set of disjoint sets; this was the number passed
    // into the constructor and is constant for the lifetime of the object. All the other methods
    // require the argument elemindex to satisfy 0 <= elemindex < number_of_elements().
    pub fn number_of_elems(&self) -> usize
    {
        self.nodes.len()
    }

    // The number of disjoint sets overall. This number decreases monotonically as time progresses;
    // each call to merge_sets() either decrements the number by one or leaves it unchanged. 0 <= number_of_sets() <= number_of_elements().
    pub fn number_of_sets(&self) -> usize
    {
        self.numberofsets
    }

    // (Private) Returns the representative element for the set containing the given element. This method is also
    // known as "find" in the literature. Also performs path compression, which alters the internal state to
    // improve the speed of future queries, but has no externally visible effect on the values returned.
    fn get_repr(&mut self, mut elemindex: usize) -> usize
    {
        // Follow parent pointers until we reach a representative
        let mut parent: usize = self.nodes[elemindex].parent;
        if parent == elemindex {
            return elemindex;
        }
        loop {
            let grandparent: usize = self.nodes[parent].parent;
            if grandparent == parent {
                return parent;
            }
            self.nodes[elemindex].parent = grandparent; // Partial path compression
            elemindex = parent;
            parent = grandparent;
        }
    }

    // Returns the size of the set that the given element is a member of. 1 <= result <= number_of_elements().
    pub fn get_size_of_set(&mut self, elemindex: usize) -> usize
    {
        let repr: usize = self.get_repr(elemindex);
        self.nodes[repr].size
    }

    // Tests whether the given two elements are members of the same set. Note that the arguments are orderless.
    pub fn are_in_same_set(&mut self, elemindex0: usize, elemindex1: usize) -> bool
    {
        self.get_repr(elemindex0) == self.get_repr(elemindex1)
    }

    // Merges together the sets that the given two elements belong to. This method is also known as "union" in the literature.
    // If the two elements belong to different sets, then the two sets are merged and the method returns true.
    // Otherwise they belong in the same set, nothing is changed and the method returns false. Note that the arguments are orderless.
    pub fn merge_sets(&mut self, elemindex0: usize, elemindex1: usize) -> bool
    {
        //debug!("Merging {} and {}", elemindex0, elemindex1);

        // Get representatives
        let mut repr0: usize = self.get_repr(elemindex0);
        let mut repr1: usize = self.get_repr(elemindex1);
        if repr0 == repr1 {
            return false;
        }

        // Compare ranks
        let cmp: i8 = self.nodes[repr0].rank - self.nodes[repr1].rank;
        if cmp == 0 {
            // Increment repr0's rank if both nodes have same rank
            self.nodes[repr0].rank += 1;
        } else if cmp < 0 {
            // Swap to ensure that repr0's rank >= repr1's rank
            std::mem::swap(&mut repr0, &mut repr1);
        }

        // Graft repr1's subtree onto node repr0
        self.nodes[repr1].parent = repr0;
        self.nodes[repr0].size += self.nodes[repr1].size;
        self.nodes[repr1].size = 0;
        self.numberofsets -= 1;
        true
    }

    // For unit tests. This detects many but not all invalid data structures, panicking if a
    // structural invariant is known to be violated. This always returns silently on a valid object.
    pub fn check_structure(&self)
    {
        let mut numrepr: usize = 0;
        for (i, node) in self.nodes.iter().enumerate() {
            let isrepr: bool = node.parent == i;
            numrepr += isrepr as usize;
            assert!(node.parent < self.nodes.len());
            assert!(0 <= node.rank && (isrepr || node.rank < self.nodes[node.parent].rank));
            assert!(!isrepr && node.size == 0 || isrepr && node.size >= (1usize << node.rank));
        }
        assert_eq!(self.numberofsets, numrepr);
        assert!(self.numberofsets <= self.nodes.len());
    }
}

/*
 * Disjoint-set data structure - Test suite (Rust)
 *
 * Copyright (c) 2018 Project Nayuki. (MIT License)
 * https://www.nayuki.io/page/disjoint-set-data-structure
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 * - The above copyright notice and this permission notice shall be included in
 *   all copies or substantial portions of the Software.
 * - The Software is provided "as is", without warranty of any kind, express or
 *   implied, including but not limited to the warranties of merchantability,
 *   fitness for a particular purpose and noninfringement. In no event shall the
 *   authors or copyright holders be liable for any claim, damages or other
 *   liability, whether in an action of contract, tort or otherwise, arising from,
 *   out of or in connection with the Software or the use or other dealings in the
 *   Software.
 */

/*---- Main runner ----*/

#[cfg(test)]
mod test_disjointset
{

    //use rand::distributions::IndependentSample;
    use rand::distributions::Distribution;
    use rand::Rng;
    //	use rand::prelude::*;
    use super::*;

    /*---- Test suite ----*/
    #[test]
    fn test_new()
    {
        let mut ds = DisjointSet::new(10);
        assert_eq!(10, ds.number_of_sets());
        assert_eq!(1, ds.get_size_of_set(0));
        assert_eq!(1, ds.get_size_of_set(2));
        assert_eq!(1, ds.get_size_of_set(9));
        assert_eq!(true, ds.are_in_same_set(0, 0));
        assert_eq!(false, ds.are_in_same_set(0, 1));
        assert_eq!(false, ds.are_in_same_set(9, 3));
    }

    #[test]
    fn test_merge()
    {
        let mut ds = DisjointSet::new(10);
        assert_eq!(true, ds.merge_sets(0, 1));
        ds.check_structure();
        assert_eq!(9, ds.number_of_sets());
        assert_eq!(true, ds.are_in_same_set(0, 1));

        assert_eq!(true, ds.merge_sets(2, 3));
        ds.check_structure();
        assert_eq!(8, ds.number_of_sets());
        assert_eq!(true, ds.are_in_same_set(2, 3));

        assert_eq!(false, ds.merge_sets(2, 3));
        ds.check_structure();
        assert_eq!(8, ds.number_of_sets());
        assert_eq!(false, ds.are_in_same_set(0, 2));

        assert_eq!(true, ds.merge_sets(0, 3));
        ds.check_structure();
        assert_eq!(7, ds.number_of_sets());
        assert_eq!(true, ds.are_in_same_set(0, 2));
        assert_eq!(true, ds.are_in_same_set(3, 0));
        assert_eq!(true, ds.are_in_same_set(1, 3));
    }

    #[test]
    fn test_big_merge()
    {
        let maxrank: i8 = 20;
        let trials: i32 = 10000;

        let numelems: usize = 1 << maxrank as usize; // Grows exponentially
        let mut ds = DisjointSet::new(numelems);
        let mut rng = rand::thread_rng();

        let range = rand::distributions::Uniform::from(0..numelems);
        for level in 0..maxrank {
            let mergestep: usize = 1 << level as usize;
            let incrstep: usize = mergestep * 2;
            let mut i: usize = 0;
            while i < numelems {
                assert_eq!(false, ds.are_in_same_set(i, i + mergestep));
                assert_eq!(true, ds.merge_sets(i, i + mergestep));
                i += incrstep;
            }
            // Now we have a bunch of sets of size 2^(level+1)

            // Do random tests
            let mask: usize = incrstep.wrapping_neg(); // 0b11...100...00
            for _ in 0..trials {
                let j: usize = range.sample(&mut rng);
                let k: usize = range.sample(&mut rng);
                let expect: bool = (j & mask) == (k & mask);
                assert_eq!(expect, ds.are_in_same_set(j, k));
            }
        }
    }

    #[test]
    fn test_against_naive_randomly()
    {
        let trials: i32 = 10;
        let iterations: i32 = 3000;
        let numelems: usize = 300;

        let mut rng = rand::thread_rng();
        let range = rand::distributions::Uniform::from(0..numelems);
        for _ in 0..trials {
            //println!("Trial {}", t);
            let mut nds = NaiveDisjointSet::new(numelems);
            let mut ds = DisjointSet::new(numelems);
            for _ in 0..iterations {
                let i: usize = range.sample(&mut rng);
                let j: usize = range.sample(&mut rng);
                assert_eq!(nds.get_size_of_set(i), ds.get_size_of_set(i));
                assert_eq!(nds.are_in_same_set(i, j), ds.are_in_same_set(i, j));
                if rng.gen::<f64>() < 0.1f64 {
                    assert_eq!(nds.merge_sets(i, j), ds.merge_sets(i, j));
                }
                assert_eq!(nds.number_of_sets(), ds.number_of_sets());
                if rng.gen::<f64>() < 0.001f64 {
                    ds.check_structure();
                }
            }
            ds.check_structure();
        }
    }

    /*---- Helper definitions ----*/

    struct NaiveDisjointSet
    {
        representatives: Vec<usize>,
    }

    impl NaiveDisjointSet
    {
        fn new(numelems: usize) -> Self
        {
            NaiveDisjointSet {
                representatives: (0usize..numelems).collect(),
            }
        }

        fn number_of_sets(&self) -> usize
        {
            self.representatives
                .iter()
                .enumerate()
                .filter(|irepr: &(usize, &usize)| *irepr.1 == irepr.0)
                .count()
        }

        fn get_size_of_set(&self, elemindex: usize) -> usize
        {
            let repr: usize = self.representatives[elemindex];
            self.representatives.iter().filter(|r| **r == repr).count()
        }

        fn are_in_same_set(&self, elemindex0: usize, elemindex1: usize) -> bool
        {
            self.representatives[elemindex0] == self.representatives[elemindex1]
        }

        fn merge_sets(&mut self, elemindex0: usize, elemindex1: usize) -> bool
        {
            let repr0: usize = self.representatives[elemindex0];
            let repr1: usize = self.representatives[elemindex1];
            for c in self.representatives.iter_mut() {
                if *c == repr1 {
                    *c = repr0;
                }
            }
            repr0 != repr1
        }
    }
}
