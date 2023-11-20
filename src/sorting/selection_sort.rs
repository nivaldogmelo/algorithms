pub fn sort(v: &mut Vec<i64>) {
    let mut min;
    let mut tmp;

    for i in 0..v.len() {
	min = v[i];

	for j in i..v.len() {
	    if v[j] < min {
		tmp = min;
		min = v[j];
		v[j] = tmp;
	    }
	}

	v[i] = min;
    }
}
