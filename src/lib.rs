pub fn a_rec(m: u64, n: u64) -> u64 {
    match (m, n) {
        (0, n) => n + 1,
        (m, 0) => a_rec(m - 1, 1),
        (m, n) => a_rec(m - 1, a_rec(m, n - 1)),
    }
}

fn a2(m: u64, n: u64) -> u64 {
    if m == 0 {
        n + 1
    } else if n == 0 {
        a_rec(m - 1, 1)
    } else {
        let l = a_rec(m, n - 1);
        a_rec(m - 1, l)
    }
}

struct Frame {
    m: u64,
    n: u64,
    location: usize,
}
pub fn a_iter(m: u64, n: u64) -> u64 {
    let mut frames = vec![Frame { m, n, location: 0 }];
    let mut values = vec![];
    while let Some(Frame { m, n, location }) = frames.pop() {
        match location {
            0 => {
                if m == 0 {
                    values.push(n + 1);
                    if frames.is_empty() {
                        break;
                    }
                } else if n == 0 {
                    frames.push(Frame {
                        m: m - 1,
                        n: 1,
                        location: 0,
                    });
                } else {
                    frames.push(Frame { m, n, location: 1 });
                    frames.push(Frame {
                        m,
                        n: n - 1,
                        location: 0,
                    });
                }
            }
            1 => {
                frames.push(Frame {
                    m: m - 1,
                    n: values.pop().unwrap(),
                    location: 0,
                });
            }
            _ => unreachable!(),
        }
    }
    values.pop().unwrap()
}
