use std::io;
use std::str;
use std::io::BufReader;

fn merge(l: Vec<i64>, r: Vec<i64>) -> Vec<i64> {
    let mut res = Vec::with_capacity(l.len() + r.len());
    let mut i = 0 as usize;
    let mut j = i;

    while i + j < l.len() + r.len() {
        if i != l.len() && (j == r.len() || l[i] < r[j]) {
            res.push(l[i]);
            i += 1;
        }
        else {
            res.push(r[j]);
            j += 1;
        }
    }

    return res;
}

fn sorted(v: Vec<i64>) -> Vec<i64> {
    let n = v.len();
    if n <= 1 {
        return v;
    }

    let m = n / 2;

    let sorted_l = sorted((&v[..m]).to_vec());
    let sorted_r = sorted((&v[m..]).to_vec());

    merge(sorted_l, sorted_r)
}


/// Reads white-space separated tokens one at a time.

pub struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: vec![],
        }
    }

    /// Use "turbofish" syntax token::<T>() to select data type of next token.
    ///
    /// # Panics
    ///
    /// Panics if there's an I/O error or if the token cannot be parsed as T.
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}


fn main() {
    let mut scanner = Scanner::new(BufReader::new(io::stdin()));


    let n: usize = scanner.token();
    let mut data = Vec::with_capacity(n);

    for _ in 0..n {
        data.push(scanner.token());
    }

    let sorted = sorted(data);
    let string_vec: Vec<String> = sorted.iter().map(|int| int.to_string()).collect();
    let output_string = string_vec.join(" ");

    println!("{}", output_string);
}
