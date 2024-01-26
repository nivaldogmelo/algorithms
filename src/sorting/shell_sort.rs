pub fn sort<T: Ord>(v: &mut Vec<T>) {
    let mut h: usize = 1;
    let lenght = v.len();

    while h < lenght / 3 {
	h = h * 3 + 1;
    }

    while h >= 1 {
	for i in h..lenght {
	    let mut j: usize = i;
	    while j >= h && v[j] < v[j - h] {
		v.swap(j, j - h);
		j = j - h;
	    }
	}
	h = h / 3;
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
