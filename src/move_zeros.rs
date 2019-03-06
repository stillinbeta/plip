#[cfg(test)]
mod test {
    #[test]
    fn example() {
        let mut input = [0, 1, 0, 3, 0, 0, 0, 12];
        super::move_zeroes(&mut input);
        assert_eq!([1, 3, 12, 0, 0, 0, 0, 0], input);
    }

    #[test]
    fn all_zeroes() {
        let mut input = [0, 0, 0];
        super::move_zeroes(&mut input);
        assert_eq!([0, 0, 0], input);
    }

    #[test]
    fn no_zeroes() {
        let mut input = [17, 1, 5];
        super::move_zeroes(&mut input);
        assert_eq!([17, 1, 5], input);
    }

    #[test]
    fn two_long() {
        let mut input = [1, 0];
        super::move_zeroes(&mut input);
        assert_eq!([1, 0], input);
    }
}

pub fn move_zeroes(arr: &mut [u32]) {
    let mut leftmost = match arr.iter().position(|x| *x == 0) {
        Some(l) => l,
        None => return,
    };
    let mut cursor = match arr.iter().position(|x| *x != 0) {
        Some(v) => v,
        None => return,
    };

    while cursor < arr.len() {
        println!("cursor: {}, lm: {}, {:?}", cursor, leftmost, arr);
        if arr[cursor] != 0 && cursor > leftmost {
            arr.swap(cursor, leftmost);
            leftmost = leftmost + 1;
        }
        cursor += 1
    }
    // [0, 1, 0, 3, 12] cursor 1 lm 0
    // [1, 0, 0, 3, 12] cursor 2 lm 1
    // [1, 0, 0, 3, 12] cursor 3 lm 1
    // [1, 3, 0, 0, 12] cursor 4 lm 2
    // [1, 3, 12, 0, 0] cursor 5 lm 3
}
