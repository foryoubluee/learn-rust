// pub fn docs() {
// 	let signed: i8 = -10;
// 	let unsigned: u8 = 10;

// 	let letter: &str = "c";
//   	let emoji: &str = "😀";

//   	let arr = [1, 2, 3, 4 ,5];
// 	let slices = &arr[1 .. 3];
//   	let tuple: (u8, &str) = (1, "a");
	
//   	let mut string: String = String::from("anjay");

// 	let array: [u8; 3] = [1, 2, 3];
// 	println!("Array: {:#?}", array)
// }

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


// fn main() {
// 	let sena = Agent {
// 		name: String::from("sena"),
// 		damage: 100,
// 		slogan: String::from("Let them die!"),
// 		agentType: AgentType::Ground
// 	};
// 	sena.say_slogan();
// 	sena.agent_info();
// }

// #[derive(Debug)]
// enum AgentType { Ground, Air, Sea }

// struct Agent {
// 	name: String,
// 	damage: i32,
// 	slogan: String,
// 	agentType: AgentType
// }

// impl Agent {
// 	fn agent_info(&self) {
// 		println!("Agent info: {}, {} ,{}, {:?}", self.name, self.damage, self.slogan, self.agentType);
// 		self.agent_level();
// 	}
// 	fn say_slogan(&self) {
// 		println!("Agent slogan: {}", self.slogan)
// 	}
// 	fn agent_level(&self) {
// 		match self.damage {
// 			0..=50 => println!("level: Low damage agent!"),
// 			100..=150 => println!("level: High damage agent!"),
// 			_ => println!("level: default"),
// 	};
// 	}
// }

// trait AgentType {
// 	fn agent_type(t: &String) -> String;
// 	fn is_human() -> bool;
// 	fn is_animal() -> bool;
// }