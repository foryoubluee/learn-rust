fn learned() {
	let player1 = User {
		id: 1,
		name: String::from("sena"),
		rank: Rank::Diamond(1)
	};
	let player2 = User {
		id: 2,
		name: String::from("seba"),
		rank: Rank::Gold(3)
	};
	let player3 = User {
		id: 3,
		name: String::from("senba"),
		rank: Rank::Silver(3)
	};

	player1.user_info();
	player2.user_info();
	player3.user_info();
}

#[derive(Debug)]
enum Rank { Diamond(u8), Gold(u8), Silver(u8) }

struct User {
	id: u32,
	name: String,
	rank: Rank
}

impl User {
	fn user_info(&self) {
		println!("INFO! Id: {:?}, Name: {:?}, Rank: {:?}", self.id, self.name, self.rank);
		match self.rank {
			Rank::Silver(1..=3) => println!("Beginner"),
			Rank::Gold(1..=3) => println!("Advance"),
			Rank::Diamond(1..=3) => println!("Pro"),
			_ => println!("Invalid rank!")
		}
	}
}
