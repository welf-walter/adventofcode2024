type Number = u64;

fn mix(a: Number, b: Number) -> Number {
    a ^ b
}

fn prune(a: Number) -> Number {
    a % 16777216
}

fn next(a: Number) -> Number {
    let b = prune(mix(a * 64, a));
    let c = prune(mix(b / 32, b));
    let d = prune(mix(c * 2048, c));
    d
}

struct Secret {
    number:Number
}

fn secret(number: Number) -> Secret { Secret { number } }

impl Iterator for Secret {
    // We can refer to this type using Self::Item
    type Item = Number;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        self.number = next(self.number);
        Some(self.number)
    }
}

#[test]
fn test_iterator() {
    assert_eq!(mix(42, 15), 37);
    assert_eq!(prune(100000000), 16113920);

    assert_eq!(secret(123).take(10).collect::<Vec<_>>(), 
      vec![15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254])
}