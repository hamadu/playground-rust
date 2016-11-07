struct SegmentTree {
    size: usize,
    data: Vec<i32>
}

impl SegmentTree {
    fn new(v: Vec<i32>) -> SegmentTree {
        let size = v.len();

        SegmentTree { size: io::stdin(), position: 0, buffer: Vec::new() }
    }

    fn read(&mut self) -> u8 {
        if self.position == self.buffer.len() {
            self.position = 0;

            let mut line = String::new();
            let _ = self.source.read_line(&mut line);
            self.buffer = line.into_bytes();

            println!("buf : {:?}", self.buffer);
        }
        self.position += 1;
        self.buffer[self.position-1]
    }

    fn next_int(&mut self) -> i32 {
        let mut c = self.read();
        loop {
            if !is_space(c) {
                break;
            }
            c = self.read();
        }

        let mut ret: i32 = 0;
        loop {
            if !is_numeric(c) {
                break;
            } else {
                ret *= 10;
                ret += (c - b'0') as i32;
            }
            c = self.read();
        }
        ret
    }
}


fn main() {


}
