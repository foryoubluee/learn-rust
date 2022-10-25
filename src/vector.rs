fn vector() {
	let mut vector: Vec<i32> = Vec::new();
	let mut macro_vector = vec![2, 5, 6];

	// Push | Adding elements
	vector.push(120); // Pushed to last index of this vector
	macro_vector.push(121); // Pushed to last index of this vector	

	// Remove | Remove elements
	vector.remove(0); // Remove by index
	macro_vector.remove(3); // Remove by index

	// Pop | Remove last elements
	vector.pop(); // remove last elements
	macro_vector.pop(); // remove last elements

	// Different way to declare a vector
	const DEFAULT: u8 = 2;
	let mut numbers = vec![DEFAULT; 3];

	numbers[2] = 5;

	for number in numbers.iter() {
		println!("{:?}", number * number);
	};

	println!("{:?}", vector);
	println!("{:?}", macro_vector);
	// println!("{:?}", number);
}