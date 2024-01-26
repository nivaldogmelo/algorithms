pub fn sort<T: Ord>(v: &mut Vec<T>) {
    for i in 1..v.len() {
	let mut j = i;
	while j > 0 && v[j] < v[j - 1] {
	    v.swap(j, j - 1);
	    j = j - 1;
	}
    }
}

#[cfg(test)]
mod tests {
    use super::sort;

    #[test]
    fn basic() {
	let mut res = vec!["d", "b", "c", "a", "e"];
	let mut clone = res.clone();

	sort(&mut res);
	clone.sort();
	assert_eq!(clone, res);
    }
}
