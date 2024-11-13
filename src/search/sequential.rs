pub fn sequential_search<T: PartialEq>(list: &[T], item: T) -> Option<usize> {
    (0..list.len()).find(|&i| list[i] == item)
}

#[cfg(test)]
mod tests {
    use super::sequential_search;

    use quickcheck::QuickCheck;
    use rand::Rng;

    fn prop_number(mut data: Vec<i32>) -> Result<(), String> {
	if !data.is_empty() {
	    data.sort();
	    data.dedup();
	    let index = rand::thread_rng().gen_range(0..data.len());
	    let item = data[index];

	    let match_index = sequential_search(&data, item);

	    match match_index == Some(index) {
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
	    match sequential_search(&data, 0) {
		None => Ok(()),
		Some(i) => Err(format!("Search mismatch: expected None, got {:?}", i)),
	    }
	}
    }

    fn prop_string(mut data: Vec<String>) -> Result<(), String> {
	if !data.is_empty() {
	    data.sort();
	    data.dedup();
	    let index = rand::thread_rng().gen_range(0..data.len());
	    let item = &data[index];

	    let match_index = sequential_search(&data, item.to_string());

	    match match_index == Some(index) {
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
	    match sequential_search(&data, "".to_string()) {
		None => Ok(()),
		Some(i) => Err(format!("Search mismatch: expected None, got {:?}", i)),
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
