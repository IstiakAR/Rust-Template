//! Fenwick tree (Binary Indexed Tree) variants.

/// Point-add, prefix-sum Fenwick tree over i64 values.
///
/// Indices are 0-based for public methods.
pub struct Fenwick {
    n: usize,
    bit: Vec<i64>,
}

impl Fenwick {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            bit: vec![0; n + 1],
        }
    }

    pub fn from_slice(values: &[i64]) -> Self {
        let mut fw = Self::new(values.len());
        for (i, &v) in values.iter().enumerate() {
            fw.add(i, v);
        }
        fw
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    /// Adds delta to values[idx].
    pub fn add(&mut self, idx: usize, delta: i64) {
        assert!(idx < self.n);
        let mut i = idx + 1;
        while i <= self.n {
            self.bit[i] += delta;
            i += i & (!i + 1);
        }
    }

    /// Returns sum(values[0..=idx]).
    pub fn prefix_sum(&self, idx: usize) -> i64 {
        assert!(idx < self.n);
        let mut i = idx + 1;
        let mut ans = 0;
        while i > 0 {
            ans += self.bit[i];
            i &= i - 1;
        }
        ans
    }

    /// Returns sum(values[l..=r]). Returns 0 when l > r.
    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l > r {
            return 0;
        }
        assert!(r < self.n);
        if l == 0 {
            self.prefix_sum(r)
        } else {
            self.prefix_sum(r) - self.prefix_sum(l - 1)
        }
    }

    /// Returns the smallest index i such that prefix_sum(i) >= target.
    ///
    /// Returns None if target <= 0 or target > total_sum.
    /// This method assumes all point updates are non-negative.
    pub fn lower_bound_prefix(&self, target: i64) -> Option<usize> {
        if self.n == 0 || target <= 0 {
            return None;
        }
        let total = self.prefix_sum(self.n - 1);
        if target > total {
            return None;
        }

        let mut idx = 0usize;
        let mut acc = 0i64;
        let mut step = 1usize;
        while (step << 1) <= self.n {
            step <<= 1;
        }

        let mut k = step;
        while k > 0 {
            let nxt = idx + k;
            if nxt <= self.n && acc + self.bit[nxt] < target {
                idx = nxt;
                acc += self.bit[nxt];
            }
            k >>= 1;
        }
        Some(idx)
    }
}

/// Range-add, point-query Fenwick wrapper over i64 values.
///
/// Indices are 0-based and inclusive in add_range.
pub struct RangeAddPointQuery {
    diff: Fenwick,
}

impl RangeAddPointQuery {
    pub fn new(n: usize) -> Self {
        Self {
            diff: Fenwick::new(n),
        }
    }

    /// Adds delta to all values in [l, r].
    pub fn add_range(&mut self, l: usize, r: usize, delta: i64) {
        if l > r {
            return;
        }
        assert!(r < self.diff.len());
        self.diff.add(l, delta);
        if r + 1 < self.diff.len() {
            self.diff.add(r + 1, -delta);
        }
    }

    /// Returns current value at index idx.
    pub fn point_query(&self, idx: usize) -> i64 {
        self.diff.prefix_sum(idx)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fenwick_sum_queries() {
        let mut fw = Fenwick::from_slice(&[3, 1, 4, 1, 5, 9]);
        assert_eq!(fw.prefix_sum(0), 3);
        assert_eq!(fw.prefix_sum(5), 23);
        assert_eq!(fw.range_sum(2, 4), 10);

        fw.add(3, 6);
        assert_eq!(fw.range_sum(2, 4), 16);
    }

    #[test]
    fn test_fenwick_lower_bound() {
        let fw = Fenwick::from_slice(&[2, 0, 3, 4, 1]);
        assert_eq!(fw.lower_bound_prefix(1), Some(0));
        assert_eq!(fw.lower_bound_prefix(2), Some(0));
        assert_eq!(fw.lower_bound_prefix(3), Some(2));
        assert_eq!(fw.lower_bound_prefix(5), Some(2));
        assert_eq!(fw.lower_bound_prefix(6), Some(3));
        assert_eq!(fw.lower_bound_prefix(10), Some(4));
        assert_eq!(fw.lower_bound_prefix(11), None);
    }

    #[test]
    fn test_range_add_point_query() {
        let mut raq = RangeAddPointQuery::new(8);
        raq.add_range(2, 5, 10);
        raq.add_range(0, 3, -3);

        assert_eq!(raq.point_query(0), -3);
        assert_eq!(raq.point_query(1), -3);
        assert_eq!(raq.point_query(2), 7);
        assert_eq!(raq.point_query(3), 7);
        assert_eq!(raq.point_query(4), 10);
        assert_eq!(raq.point_query(5), 10);
        assert_eq!(raq.point_query(6), 0);
    }
}