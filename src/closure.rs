// Closure | Anonymous fn | Lambda fn

fn closure() {
	let onPercentage = |val: f32, percent: f32| -> f32 {
		let percentage = percent / 100.0;
		let result = percentage * val;
		result
	};
	println!("{}", onPercentage(7000.0, 50.0))
}