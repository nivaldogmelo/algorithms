use std::fs::{read_to_string, File};

use std::io::{BufRead, BufReader, Error, ErrorKind};

mod data_structures;
mod sorting;
mod union_find;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
	result.push(line.to_string())
    }

    result
}

fn load_from_file(file_path: &str) -> Result<Vec<i64>, Error> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let mut numbers = Vec::new();

    for line in reader.lines() {
	numbers.push(
	    line?
		.trim()
		.parse()
		.map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
	);
    }

    Ok(numbers)
}

fn main() {
    ///////////////////////////////////////////////////////////////////////////
    //             tiny - 10, medium 625, large 1000000, ularge               //
    ///////////////////////////////////////////////////////////////////////////
    let input = read_lines("mediumUF.txt");

    let mut union = union_find::union_find::UF::new(625);

    for i in input {
	let mut n = i.split_whitespace();
	let tuple = (
	    n.next().unwrap().parse::<i32>().unwrap(),
	    n.next().unwrap().parse::<i32>().unwrap(),
	);

	if union.connected(tuple.0, tuple.1) {
	    continue;
	} else {
	    union.union(tuple.0, tuple.1);
	    println!("{} {}", tuple.0, tuple.1);
	}
    }

    println!("{} components", union.count());

    let mut input = load_from_file("1Mints.txt");
    // let mut input = vec![4, 3, 6, 5, 7, 9, 1, 12, 14, 15, 11, 8, 0, 10, 13, 2];

    // sorting::bu_merge_sort::merge_sort(&mut input);
    sorting::bu_merge_sort::merge_sort(&mut input.as_mut().unwrap());
    println!("{:?}", Some(input));
}
