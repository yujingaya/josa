#[cfg(test)]
mod tests {
	#[test]
	fn basic() {
		use josa::Josa::{EunNeun, IGa};
		use josa::JosaExt;

		let mut user = "유진".to_owned();
		let mut mackerel = "고등어".to_owned();

		user.push_josa(EunNeun);
		mackerel.push_josa(IGa);

		let sentence = format!("{} {} 먹고싶다", user, mackerel);

		assert_eq!(sentence, "유진은 고등어가 먹고싶다");
	}
}
