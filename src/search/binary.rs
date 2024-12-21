pub fn binary_search<T: PartialOrd>(data: &[T], item: T) -> usize {
    if data.is_empty() {
	return 0;
    }
    search(data, item, 0, data.len() - 1)
}

fn search<T: std::cmp::PartialOrd>(data: &[T], item: T, lo: usize, hi: usize) -> usize {
    if hi < lo {
	return lo;
    }

    let mid = lo + (hi - lo) / 2;
    if item < data[mid] {
	search(data, item, lo, mid - 1)
    } else if item > data[mid] {
	search(data, item, mid + 1, hi)
    } else {
	mid
    }
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    use quickcheck::QuickCheck;
    use rand::Rng;

    fn prop_number(mut data: Vec<i32>) -> Result<(), String> {
	if !data.is_empty() {
	    data.sort();
	    data.dedup();
	    let index = rand::thread_rng().gen_range(0..data.len());
	    let item = data[index];

	    let match_index = binary_search(&data, item);

	    match match_index == index {
		true => Ok(()),
		false => {
		    return Err(format!(
			"Search mismatch: expected {:?}, got {:?}",
			Some(index),
			match_index
		    ))
		}
	    }
	} else {
	    let match_index = binary_search(&data, 0);
	    match match_index == 0 {
		true => Ok(()),
		false => Err(format!(
		    "Search mismatch: expected None, got {:?}",
		    match_index
		)),
	    }
	}
    }

    fn prop_string(mut data: Vec<String>) -> Result<(), String> {
	if !data.is_empty() {
	    data.sort();
	    data.dedup();
	    let index = rand::thread_rng().gen_range(0..data.len());
	    let item = &data[index];

	    let match_index = binary_search(&data, item.to_string());

	    match match_index == index {
		true => Ok(()),
		false => {
		    return Err(format!(
			"Search mismatch: expected {:?}, got {:?}",
			Some(index),
			match_index
		    ))
		}
	    }
	} else {
	    let match_index = binary_search(&data, "".to_string());
	    match match_index == 0 {
		true => Ok(()),
		false => Err(format!(
		    "Search mismatch: expected None, got {:?}",
		    match_index
		)),
	    }
	}
    }

    #[test]
    fn quickcheck_props() {
	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_number as fn(Vec<i32>) -> Result<(), String>);

	QuickCheck::new()
	    .tests(1000)
	    .quickcheck(prop_string as fn(Vec<String>) -> Result<(), String>);
    }
}
