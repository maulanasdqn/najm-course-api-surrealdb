use rand::{Rng, rng};

pub struct OtpManager;

impl OtpManager {
	pub fn generate_otp() -> u32 {
		rng().random_range(100_000..1_000_000)
	}

	pub fn validate_otp(stored_otp: u32, user_otp: u32) -> bool {
		stored_otp == user_otp
	}
}

#[cfg(test)]
mod generate_otp_test {
	use super::*;

	#[test]
	fn test_generate_otp_should_return_u32() {
		let otp = OtpManager::generate_otp();
		assert!(otp > 0);
	}

	#[test]
	fn test_validate_otp_should_return_true() {
		let otp = OtpManager::generate_otp();
		let valid = OtpManager::validate_otp(otp, otp);
		assert!(valid);
	}

	#[test]
	fn test_validate_otp_should_return_false() {
		let otp = OtpManager::generate_otp();
		let valid = OtpManager::validate_otp(otp, otp + 1);
		assert!(!valid);
	}
}
