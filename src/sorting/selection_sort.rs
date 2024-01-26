pub fn sort<T: Ord>(v: &mut Vec<T>) {
    let mut min;

    for i in 0..(v.len()) {
	min = i;

	for j in i..v.len() {
	    if v[j] < v[min] {
		v.swap(j, min);
	    }
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
