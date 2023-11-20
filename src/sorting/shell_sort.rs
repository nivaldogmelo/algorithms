pub fn sort(v: &mut Vec<i64>) {
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
