// macro_rules! hello {
// 	() => {println!("hello")}
// }

// macro_rules! square {
// 	($square: expr) => {
// 		println!("{}", $square * $square)
// 	};
// }

// macro_rules! name {
// 	($($name: expr), *) => (
// 		$(println!("{}", $name);) *
// 	);
// }

// macro_rules! create_fn {
// 	($fn_name: ident) => {
// 		fn $fn_name<T>(x: T, y: T) -> [T; 2] {
// 			[x, y]
// 		}
// 	}
// }

// fn main() {
// 	name!("john", "tereecnce");
// 	create_fn!(arr);
// 	let a = arr::<i32>(20, 20);
// 	println!("{:?}", a)
// }
