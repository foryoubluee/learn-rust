// // Trait
// #[derive(Debug)]
// struct User {
// 	username: String,
// 	email: String,
// 	password: String
// }

// trait Register {
// 	fn print_data(&self) -> Self;
// }

// impl Register for User {
// 	fn print_data(&self) -> Self {
// 		println!("{:#?}", &self);
// 		return User {
// 			username: self.username.to_string(),
// 			email: self.email.to_string(),
// 			password: self.password.to_string()
// 		};
// 	}
// }

// fn main() {
// 	let user1 = User {
// 		username: "bent".to_string(),
// 		email: "ben@gmail.com".to_string(),
// 		password: "0x987eaf7123".to_string()
// 	};
// 	user1.print_data();
// }