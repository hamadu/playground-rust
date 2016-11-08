use std::cmp::min;

trait Monoid {
    fn unit() -> Self;

    fn merge(a: Self, b: Self) -> Self;
}

struct Rational {
    p: i32,
    q: i32
}

struct SegmentTree<T> {
    m: usize,
    data: Vec<T>
}

impl<T: Monoid> SegmentTree<T>  {
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

    fn min(& self, range: (usize, usize)) -> T {
        self.min_range(range, 0, (0, self.m))
    }

    fn min_range(& self, (query_l, query_r): (usize, usize), idx: usize, (seg_l, seg_r): (usize, usize)) -> T {
        if query_r <= seg_l || seg_r <= query_l {
            return T::unit();
        }
        if query_l <= seg_l && seg_r <= query_r {
            return self.data[idx];
        }

        let seg_med = (seg_l + seg_r) / 2;
        let l = self.min_range((query_l, query_r), idx*2+1, (seg_l, seg_med));
        let r = self.min_range((query_l, query_r), idx*2+2, (seg_med, seg_r));
        return T::merge(l, r)
    }

    fn set(&mut self, index: usize, value: T) {
        let mut pos = self.m-1+index;
        self.data[pos] = value;
        while pos > 0 {
            pos = (pos-1)/2;
            self.data[pos] = self.data[pos*2+1].merge(self.data[pos*2+2]);
        }
    }
}

fn main() {
    let mut seg = SegmentTree::new(vec![7,8,1,2,4,6,3,4], 100);

    assert_eq!(seg.min((0, 4)), 1);
    assert_eq!(seg.min((3, 6)), 2);
    assert_eq!(seg.min((1, 2)), 8);

    seg.set(2, 5);
    assert_eq!(seg.min((0, 8)), 2);
    assert_eq!(seg.min((0, 3)), 5);

}
