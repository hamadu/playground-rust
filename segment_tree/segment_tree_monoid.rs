trait Monoid {
    fn unit() -> Self;

    fn merge(a: Self, b: Self) -> Self;
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Rational {
    p: i64,
    q: i64
}

impl Rational {
    fn new(p: i64, q: i64) -> Rational {
        Rational { p: p, q: q }
    }
}

impl Monoid for Rational {
    fn unit() -> Rational {
        Rational { p: 1, q: 0 }
    }

    fn merge(a: Rational, b: Rational) -> Rational {
        if a.p * b.q < a.q * b.p {
            Rational { p: a.p, q: a.q }
        } else {
            Rational { p: b.p, q: b.q}
        }
    }
}

impl Monoid for i32 {
    fn unit() -> i32 {
        1000000000
    }

    fn merge(a: i32, b: i32) -> i32 {
        if a < b {
            a
        } else {
            b
        }
    }
}

struct SegmentTree<T> {
    m: usize,
    data: Vec<T>
}

impl<T: Monoid + Copy> SegmentTree<T>  {
    fn new(v: Vec<T>) -> SegmentTree<T> {
        let size = v.len();
        let mut m = 1;
        while m < size {
            m *= 2;
        }
        let mut data: Vec<T> = Vec::new();
        for _ in 0..2*m {
            data.push(T::unit());
        }
        for i in 0..size {
            data[m-1+i] = v[i];
        }
        for i in 0..m-1 {
            let idx = m-2-i;
            data[idx] = T::merge(data[idx*2+1], data[idx*2+2]);
        }
        SegmentTree { m: m, data: data }
    }

    fn find(& self, range: (usize, usize)) -> T {
        self.find_range(range, 0, (0, self.m))
    }

    fn find_range(& self, (query_l, query_r): (usize, usize), idx: usize, (seg_l, seg_r): (usize, usize)) -> T {
        if query_r <= seg_l || seg_r <= query_l {
            return T::unit();
        }
        if query_l <= seg_l && seg_r <= query_r {
            return self.data[idx];
        }

        let seg_med = (seg_l + seg_r) / 2;
        let l = self.find_range((query_l, query_r), idx*2+1, (seg_l, seg_med));
        let r = self.find_range((query_l, query_r), idx*2+2, (seg_med, seg_r));
        return T::merge(l, r)
    }

    fn set(&mut self, index: usize, value: T) {
        let mut pos = self.m-1+index;
        self.data[pos] = value;
        while pos > 0 {
            pos = (pos-1)/2;
            self.data[pos] = T::merge(self.data[pos*2+1], self.data[pos*2+2]);
        }
    }
}

fn main() {
    // use i32
    let mut seg = SegmentTree::new(vec![
        7,8,1,2,4,6,3,4
    ]);

    assert_eq!(seg.find((0, 4)), 1);
    assert_eq!(seg.find((3, 6)), 2);
    assert_eq!(seg.find((1, 2)), 8);

    seg.set(2, 5);
    assert_eq!(seg.find((0, 8)), 2);
    assert_eq!(seg.find((0, 3)), 5);


    // use Rational
    let mut seg = SegmentTree::new(vec![
        Rational::new(1, 2),
        Rational::new(2, 3),
        Rational::new(1, 4),
        Rational::new(2, 5),
        Rational::new(1, 6),
        Rational::new(2, 7),
        Rational::new(1, 8),
        Rational::new(2, 9)
    ]);

    assert_eq!(seg.find((0, 4)), Rational::new(1, 4));
    assert_eq!(seg.find((3, 6)), Rational::new(1, 6));
    assert_eq!(seg.find((1, 2)), Rational::new(2, 3));

    seg.set(7, Rational::new(2, 21));
    assert_eq!(seg.find((0, 8)), Rational::new(2, 21));
}
