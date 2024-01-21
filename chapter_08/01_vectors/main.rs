fn dup_in_place(v: &mut Vec<i32>) {
	for i in 0..v.len() {
		v.push(v[i]);
	}
}

fn main() {
	let mut v = vec![1, 2, 3];

	v.push(4);
	v.push(5);
	v.push(6);
	v.push(7);
	v.push(8);

	let third: &i32 = &v[2];
	println!("The third element is {third}");

	let third: Option<&i32> = v.get(2);
	match third {
		Some(third) => 	println!("The third element is {third}"),
		None => println!("There is no third element"),
	}

	for n_ref in &v {
		let n_ref_plus: i32 = *n_ref + 1;
		println!("{n_ref_plus}");
	}

	for n_ref in &mut v {
		*n_ref += 50;
	}

	println!("{:?}", v);

	dup_in_place(&mut v);
	println!("{:?}", v);

	#[derive(Debug)]
	enum SpreadsheetCell {
		Int(i32),
		Float(f64),
		Text(String),
	}

	let row = vec![
		SpreadsheetCell::Int(3),
		SpreadsheetCell::Text(String::from("blue")),
		SpreadsheetCell::Float(10.12),
	];

	println!("{:?}", row);

	let mut v1: Vec<i32> = vec![1, 2, 3];
	let mut v2: Vec<&mut i32> = Vec::new();

	for i in &mut v1 {
		v2.push(i);
	}

	*v2[0] = 5;

	let a = *v2[0];
	let b = v1[0];
	println!("{a} {b}");
}