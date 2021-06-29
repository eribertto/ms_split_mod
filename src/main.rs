mod authentication {
	use core::str;
use std::collections::hash_map::DefaultHasher;
	use std::hash::{Hash, Hasher};
use std::u64;

	pub struct User {
		username: String,
		password_hash: u64,
	}

	impl User {
		pub fn new(username: &str, password: &str) -> User {
			User {
				username: username.to_string(),
				password_hash: hash_password(&password.to_owned()),
			}
		}
		pub  fn get_username(&self) -> &String {
			&self.username
		}
		pub fn set_password(&mut self, new_password: &str) {
			self.password_hash = hash_password(&new_password.to_owned())
		}
	}
	fn hash_password<T: Hash>(t: &T) -> u64 {
		let mut s = DefaultHasher::new();
		t.hash(&mut s);
		s.finish()
	}
}

fn main() {
	let mut user = authentication::User::new("jeremy", "super-secret");
	println!("The username is: {}", user.get_username());
	user.set_password("even-more-secret");
}