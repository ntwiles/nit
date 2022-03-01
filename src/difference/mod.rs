mod delta;

use std::collections::{HashMap, HashSet};

use delta::Delta;

// TODO: This is wrong, using a hashmap will not work for any lines which happen to be the same.
pub fn get_diff<'a>(
    a: impl Iterator<Item = &'a str>,
    b: impl Iterator<Item = &'a str>,
) -> Vec<Delta> {
    let hm_a: HashMap<&str, usize> = HashMap::from_iter(a.enumerate().map(|(a, b)| (b, a)));
    let hm_b: HashMap<&str, usize> = HashMap::from_iter(b.enumerate().map(|(a, b)| (b, a)));

    let hs_a: HashSet<&str> = hm_a.keys().copied().collect();
    let hs_b: HashSet<&str> = hm_b.keys().copied().collect();

    let removes = hs_a
        .difference(&hs_b)
        .map(|rm| Delta::removed(rm, hm_a[rm] + 1));

    let adds = hs_b
        .difference(&hs_a)
        .map(|add| Delta::added(add, hm_b[add] + 1));

    let mut deltas: Vec<Delta> = removes.chain(adds).collect();
    deltas.sort_by_key(|d| d.line);
    deltas
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_identifies_added_line() {
        let old = "a line\nanother line";
        let new = "a line\na diff!\nanother line";

        let expected = vec![Delta::added("a diff!", 2)];
        let actual = get_diff(old.lines(), new.lines());

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_identifies_two_sequential_added_lines() {
        let old = "a line\nanother line";
        let new = "a line\na diff!\nanother diff!\nanother line";

        let expected = vec![Delta::added("a diff!", 2), Delta::added("another diff!", 3)];
        let actual = get_diff(old.lines(), new.lines());

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_identifies_removed_line() {
        let old = "a line\nline to remove\na third line";
        let new = "a line\na third line";

        let expected = vec![Delta::removed("line to remove", 2)];
        let actual = get_diff(old.lines(), new.lines());

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_identifies_removed_last_line() {
        let old = "a line\nanother line\nremove";
        let new = "a line\nanother line";

        let expected = vec![Delta::removed("remove", 3)];
        let actual = get_diff(old.lines(), new.lines());

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_identifies_two_sequential_removed_lines() {
        let old = "a line\nremove a\nremove b\nlast line";
        let new = "a line\nlast line";

        let expected = vec![Delta::removed("remove a", 2), Delta::removed("remove b", 3)];
        let actual = get_diff(old.lines(), new.lines());

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_identifies_one_added_and_one_removed_line() {
        let old = "first line\nremove\nlast line";
        let new = "first line\nadd\nlast line";

        let expected = vec![Delta::removed("remove", 2), Delta::added("add", 2)];
        let actual = get_diff(old.lines(), new.lines());

        assert_eq!(expected, actual);
    }
}
