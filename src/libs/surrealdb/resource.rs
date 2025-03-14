use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceEnum {
	Gacha,
	GachaClaims,
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
			ResourceEnum::Gacha => "app_gacha",
			ResourceEnum::GachaClaims => "app_gacha_claims",
		};
		write!(f, "{}", str)
	}
}
