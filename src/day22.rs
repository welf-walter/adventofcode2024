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

type Price = i32;
type Changes = [Price; 4];
const INVALID_CHANGE : Price = -99;

struct PriceIter {
    secret:Secret,
    changes:Changes
}

fn prices(secret: Secret) -> PriceIter { PriceIter { secret, changes:[INVALID_CHANGE, INVALID_CHANGE, INVALID_CHANGE, INVALID_CHANGE] } }

fn price(number: Number) -> Price { ( number % 10 ) as Price }

impl Iterator for PriceIter {
    // We can refer to this type using Self::Item
    type Item = (Price, Changes);

    fn next(&mut self) -> Option<Self::Item> {
        let old = price(self.secret.number);
        let new = price(self.secret.next().unwrap());
        self.changes[0] = self.changes[1];
        self.changes[1] = self.changes[2];
        self.changes[2] = self.changes[3];
        self.changes[3] = new - old;
        Some((new, self.changes))
    }
}

fn monkey_deal(secret:Secret, changes_to_sell:Changes) -> Price {
    match prices(secret).take(2000).find(|price_and_change| price_and_change.1 == changes_to_sell) {
        None => 0,
        Some(price_and_change) => price_and_change.0
    }
}

struct AllPossibleChangesIterator {
    changes:Changes
}

impl Iterator for AllPossibleChangesIterator {
    // We can refer to this type using Self::Item
    type Item = Changes;

    fn next(&mut self) -> Option<Self::Item> {
        self.changes[0] += 1;
        if self.changes[0] < 10 { return Some(self.changes); }

        self.changes[0] = -9;
        self.changes[1] += 1;
        if self.changes[1] < 10 { return Some(self.changes); }

        self.changes[1] = -9;
        self.changes[2] += 1;
        if self.changes[2] < 10 { return Some(self.changes); }

        self.changes[2] = -9;
        self.changes[3] += 1;
        if self.changes[3] < 10 { return Some(self.changes); }

        return None;
    }
}

fn all_possible_changes() -> AllPossibleChangesIterator {
    AllPossibleChangesIterator { changes: [-10, -9, -9, -9] }
}

#[test]
fn test_iterator() {
    assert_eq!(mix(42, 15), 37);
    assert_eq!(prune(100000000), 16113920);

    assert_eq!(secret(123).take(10).collect::<Vec<_>>(),
      vec![15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254]);
    assert_eq!(secret(123).nth(10-1), Some(5908254));

    assert_eq!([1, 10, 100, 2024].iter().map(|initial| secret(*initial).nth(2000-1).unwrap()).collect::<Vec<_>>(),
        vec![8685429, 4700978, 15273692, 8667524]);

    assert_eq!([1, 10, 100, 2024].iter().map(|initial| secret(*initial).nth(2000-1).unwrap()).sum::<Number>(),
        37327623);

    // part 2
    assert_eq!(secret(123).take(10).map(price).collect::<Vec<_>>(),
    vec![/* 3, */ 0, 6, 5, 4, 4, 6, 4, 4, 2, 4]);

    assert_eq!(prices(secret(123)).take(9).collect::<Vec<_>>(),
      vec![
        /* 3 */
        (0, [INVALID_CHANGE, INVALID_CHANGE, INVALID_CHANGE, -3]),
        (6, [INVALID_CHANGE, INVALID_CHANGE, -3, 6]),
        (5, [INVALID_CHANGE, -3, 6, -1]),
        (4, [-3, 6, -1, -1]),
        (4, [6, -1, -1, 0]),
        (6, [-1, -1, 0, 2]),
        (4, [-1, 0, 2, -2]),
        (4, [0, 2, -2, 0]),
        (2, [2, -2, 0, -2])
      ]);

    let changes_to_sell1 = [-1, -1, 0, 2];
    assert_eq!(monkey_deal(secret(123), changes_to_sell1), 6);

    let changes_to_sell2 = [-2, 1, -1, 3];
    assert_eq!(monkey_deal(secret(1), changes_to_sell2), 7);
    assert_eq!(monkey_deal(secret(2), changes_to_sell2), 7);
    assert_eq!(monkey_deal(secret(3), changes_to_sell2), 0);
    assert_eq!(monkey_deal(secret(2024), changes_to_sell2), 9);

    assert_eq!(all_possible_changes().take(3).collect::<Vec<_>>(),
        vec![
            [-9,-9,-9,-9],
            [-8,-9,-9,-9],
            [-7,-9,-9,-9]
        ]);

    assert_eq!(all_possible_changes().skip(18).take(3).collect::<Vec<_>>(),
        vec![
            [ 9,-9,-9,-9],
            [-9,-8,-9,-9],
            [-8,-8,-9,-9]
        ]);

    assert_eq!(all_possible_changes().count(), 19*19*19*19);

}

//////////////////////////////////////////
/// Puzzle
//////////////////////////////////////////

pub fn puzzle() {
    let lines = crate::helper::read_file("input/day22.txt");

    let initials = lines.into_iter().map(|line| line.parse::<Number>().unwrap());
    let sum:Number = initials.map(|initial| secret(initial).nth(2000-1).unwrap()).sum();

    println!("Day 22, Part 1: Sum of 2000th secret is {}", sum);

}
