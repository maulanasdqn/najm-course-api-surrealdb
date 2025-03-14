use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceEnum {
	Users,
	Roles,
	Permissions,
}

impl fmt::Display for ResourceEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let str = match self {
			ResourceEnum::Users => "app_users",
			ResourceEnum::Roles => "app_roles",
			ResourceEnum::Permissions => "app_permissions",
		};
		write!(f, "{}", str)
	}
}
