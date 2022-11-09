use std::vec;

fn main() {
	let mut i = 0;
	let mut data_mahasiswa: Vec<&Mahasiswa> = vec![];

	let sena = &Mahasiswa {
		id: 1,
		name: Name { fname: "Aryasena".to_owned(), lname: "Putra".to_owned() },
		nilai: Nilai { nilai_tugas: 90.0, nilai_uas: 50.9 }
	};

	while i < 5 {
		data_mahasiswa.push(sena);
		i+=1;
	}

	for mhs in data_mahasiswa {
		println!("{}", mhs.get_data());
	}
}
impl Print<String> for Mahasiswa {
	fn get_data(&self) -> String {
			Mahasiswa {
				id: self.id,
				name: Name { fname: self.name.fname.to_owned(), lname: self.name.lname.to_owned() },
				nilai: Nilai { nilai_tugas: self.nilai.nilai_tugas, nilai_uas: self.nilai.nilai_uas }
			};
			format!(
				"{}. Name: {} {} | Nilai: {} {}", 
				self.id,
				self.name.fname.to_owned(),
				self.name.lname.to_owned(),
				self.nilai.nilai_tugas,
				self.nilai.nilai_uas
			)
	}
}
trait Print<T> {
	fn get_data(&self) -> T;
}

struct Mahasiswa {
	id: u8,
	name: Name,
	nilai: Nilai
}

struct Name {
	fname: String,
	lname: String
}

struct Nilai {
	nilai_tugas: f32,
	nilai_uas: f32
}