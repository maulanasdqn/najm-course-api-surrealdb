use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedisKeyEnum {
	User,
	Token,
	Otp,
}

impl fmt::Display for RedisKeyEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let str = match self {
			RedisKeyEnum::User => "user",
			RedisKeyEnum::Token => "token",
			RedisKeyEnum::Otp => "otp",
		};
		write!(f, "{}", str)
	}
}
