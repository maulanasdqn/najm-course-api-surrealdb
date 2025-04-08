use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RolesEnum {
	Admin,
	Student,
	Staf,
}

impl fmt::Display for RolesEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let roles_str = match self {
			RolesEnum::Admin => "Admin",
			RolesEnum::Student => "Student",
			RolesEnum::Staf => "Staf",
		};
		write!(f, "{}", roles_str)
	}
}
