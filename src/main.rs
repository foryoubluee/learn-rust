// Generics in Rust

fn main() {
	let point: Point<f32> = Point { x: 12.3, y: 10.2 };
	point.print_point();

	let red = Color::Red("Red");
	let blue = Color::Blue(256);
	let green = Color::Green(true);

	println!("r: {:?}, g: {:?}, b: {:?}", red, green, blue);
}

#[derive(Debug)]
enum Color<T> { Red(T), Green(T), Blue(T) }

struct Point<T> {
	x: T,
	y: T
}

// struct Point<T, V> {
// 	  x: T,
// 	  y: V
// }

impl Point<f32> {
	fn print_point(&self) {
		println!("x: {:?}, y: {:?}", self.x, self.y)
	}
}