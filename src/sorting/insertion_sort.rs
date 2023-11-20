pub fn sort(v: &mut Vec<i64>) {
    for i in 1..v.len() {
	let mut j = i;
	while j > 0 && v[j] < v[j - 1] {
	    v.swap(j, j - 1);
	    j = j - 1;
	}
    }
}
