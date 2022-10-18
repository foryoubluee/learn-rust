pub fn docs() {
	let signed: i8 = -10;
	let unsigned: u8 = 10;

	let letter: &str = "c";
  	let emoji: &str = "😀";

  	let arr = [1, 2, 3, 4 ,5];
	let slices = &arr[1 .. 3];
  	let tuple: (u8, &str) = (1, "a");
	
  	let mut string: String = String::from("anjay");

	let array: [u8; 3] = [1, 2, 3];
	println!("Array: {:#?}", array)
}

// fn main() {
// 	let arr: [u8; 3] = [1, 2, 3];
// 	let slice = &arr[0 .. arr.len()];
// 	print_value(arr, slice);
// }

// fn print_value(arr: [u8; 3], slice: &[u8]) {
// 	println!("Array, slices: {:?} | {:?}", arr, slice);
// 	println!("index 1: {:?} | {:?}", arr[1], slice[1]);
// 	println!("length: {:?} | {:?}", arr.len(), slice.len());
// }

// fn main() {
// 	let mut s = String::from("hello world");
// 	s.push_str(", wotd");

// 	{
// 		let x = &s;
// 		println!("x: {}", x);
// 	}
// 	do_something(&s);
// 	do_other_something(&s);
// 	do_something(&s);
// }

// fn do_something(s: &String) {
// 	let x = s;
// 	println!("{}", x)
// }

// fn do_other_something(s: &String) {
// 	println!("{}", s)
// }