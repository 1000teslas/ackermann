use scratch::{a_rec, a_iter};
fn main() {
    assert_eq!(a_iter(4, 1), a_rec(4, 1));
    assert_eq!(a_iter(3, 3), a_rec(3, 3));
}
