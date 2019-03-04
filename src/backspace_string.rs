#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(backspace_compare("ab#c", "ad#c"))
    }

    #[test]
    fn example2() {
        assert!(backspace_compare("ab##", "c#d#"))
    }

    #[test]
    fn example3() {
        assert!(backspace_compare("a##c", "#a#c"))
    }

    #[test]
    fn remove_single() {
        assert_eq!(vec!['a', 'b', 'c'], remove_backspace("abc#c"));
        assert_eq!(vec!['a', 'b'], remove_backspace("abc#"));
    }

    #[test]
    fn remove_many() {
        assert_eq!(vec!['a', 'b', 'c'], remove_backspace("abdef###c"));
        assert_eq!(Vec::<char>::new(), remove_backspace("abc###"));
    }
}

fn remove_backspace(input: &str) -> Vec<char> {
    let mut stack = Vec::new();
    for c in input.chars() {
        match c {
            '#' => { stack.pop(); },
            c => { stack.push(c); }
        }
    }
    stack
}

#[allow(dead_code)]
fn backspace_compare(in1: &str, in2: &str) -> bool {
    let in1 = remove_backspace(in1);
    let in2 = remove_backspace(in2);

    in1 == in2
}
