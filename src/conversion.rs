// // Konversi biner => decimal | decimal => biner.
// use std::{num::ParseIntError};

// fn main() {	
// 	loop {
// 		let mut input_type = String::new();
// 		let mut input = String::new();
// 		println!("Input: decimal | binary");
		
// 		std::io::stdin().read_line(&mut input_type).unwrap();

// 		let user_inputted_type = String::from(&input_type);
		
// 		match user_inputted_type.as_str() {
// 			"decimal" => {
// 				println!("Input decimal number:");
// 				std::io::stdin().read_line(&mut input).unwrap();
// 				println!("the binary of {}, is {:?}", input, to_binary(&input));
// 				break;
// 			},
// 			"binary" => {
// 				println!("Input binary number:");
// 				std::io::stdin().read_line(&mut input).unwrap();
// 				println!("the binary of {}, is {:?}", input, to_decimal(&input));
// 				break;
// 			},
// 			_ => println!("Please input the correct phrase!, {}", user_inputted_type)
// 		}
// 	}
// }

// fn to_decimal(input: &String) -> Result<i32, ParseIntError> {
// 	let parsed_input = input.parse::<i32>()?;
// 	Ok(parsed_input)
// }
// fn to_binary(input: &String) -> Result<i32, ParseIntError> {
// 	let parsed_input = input.parse::<i32>()?;
// 	Ok(parsed_input)
// }