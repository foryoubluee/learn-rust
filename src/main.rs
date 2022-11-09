use rand::random;
use sha256::digest;

fn main() {
	let mut new_user = &mut User {
		name: "sena".to_owned(),
		email: "awdawd@sawdaw.ks".to_owned(),
		pocket: Pocket {
			pocket_addr: random_pocket_address(),
			balance: 0
		}
	};
	let mut new_user2 = &mut User {
		name: "bbb".to_owned(),
		email: "dgdfgd@sawdaw.ks".to_owned(),
		pocket: Pocket {
			pocket_addr: random_pocket_address(),
			balance: 0
		}
	};

	new_user.pocket.balance = 2;
	new_user2.pocket.balance = 3;
	println!("{:#?}", new_user);
	println!("{:#?}", new_user2);
	println!("{}", new_user.pocket.pocket_addr.len());
	println!("{}", new_user2.pocket.pocket_addr.len());
}

#[derive(Debug)]
struct Pocket {
	pocket_addr: String,
	balance: u128
}

#[derive(Debug)]
struct User {
	name: String,
	email: String,
	pocket: Pocket
}

trait Create {
	fn new(&self) -> Self;
}

impl Create for User {
	fn new(&self) -> Self {
		User { name: self.name.to_owned(), email: self.email.to_owned(), pocket: Pocket { pocket_addr: self.pocket.pocket_addr.to_owned(), balance: self.pocket.balance } }
	}
}

impl Create for Pocket {
	fn new(&self) -> Self {
		Pocket { pocket_addr: random_pocket_address(), balance: 0 }
	}
}

fn random_pocket_address() -> String {
	let mut used_addreses: Vec<String> = vec![];
	let random_number = random::<u128>();
	let hashed = hash(random_number.to_string());
	used_addreses.push(hashed.to_owned());
	println!("{:?}", used_addreses[0..1].to_owned());
	format!("0x{}", hashed[2..64].to_owned())
}

fn hash(random: String) -> String {
	let sha256 = digest(random.as_str());
	sha256
}