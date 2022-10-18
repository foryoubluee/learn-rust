
fn main() {
	let sena = Agent {
		name: String::from("sena"),
		damage: 100,
		slogan: String::from("Let them die!")
	};
	sena.say_slogan();
	sena.agent_info();
}

struct Agent {
	name: String,
	damage: i32,
	slogan: String,
	agentType: String
}

impl Agent {
	fn agent_info(&self) {
		println!("Agent info: {}, {} ,{}", self.name, self.damage, self.slogan);
		self.agent_level();
	}
	fn say_slogan(&self) {
		println!("Agent slogan: {}", self.slogan)
	}
	fn agent_level(&self) {
		match self.damage {
			0..=50 => println!("level: Low damage agent!"),
			100..=150 => println!("level: High damage agent!"),
			_ => println!("level: default"),
	};
	}
}

trait AgentType {
	fn agent_type(t: &String) -> String;
	fn is_human() -> bool;
	fn is_animal() -> bool;
}

impl AgentType for Agent {
	fn agent_type(t: &String) -> String {
		String::frm(t)
	}
}