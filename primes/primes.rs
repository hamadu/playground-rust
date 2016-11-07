use std::io;

struct Reader {
    position: usize,
    buffer: Vec<u8>,
    source: io::Stdin
}

fn is_space(input: u8) -> bool {
    input == b' ' || input == b'\n' || input == b'\t'
}

fn is_numeric(input: u8) -> bool {
    input >= b'0' && input <= b'9'
}

impl Reader {
    fn new() -> Reader { Reader { source: io::stdin(), position: 0, buffer: Vec::new() } }

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

fn is_prime(n: i32) -> bool {
    if n == 1 {
        return false;
    }

    for x in 2..n {
        if x * x > n {
            break;
        }
        if n % x == 0 {
            return false;
        }
    }
    return true;
}

fn prime(n: i32) -> i32 {
    let mut nn = n;
    while nn >= 1 {
        if is_prime(nn) {
            return nn;
        }
        nn -= 1;
    }
    -1
}

fn main() {
    let mut reader = Reader::new();

    let n: i32 = reader.next_int();

    for _ in 0..n {
        let num = reader.next_int();
        println!("prime no more than {} => {}", num, prime(num));
    }
    println!("bye");
}
