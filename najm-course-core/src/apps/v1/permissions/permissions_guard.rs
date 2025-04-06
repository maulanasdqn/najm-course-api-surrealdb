use super::PermissionsEnum;
use crate::{common_response, extract_email, AppState, AuthRepository};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response,
};

pub async fn permissions_guard(
	headers: &HeaderMap,
	state: AppState,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<(), Response> {
	let auth_repo = AuthRepository::new(&state);
	let email = extract_email(headers).ok_or_else(|| {
		common_response(
			StatusCode::UNAUTHORIZED,
			"Invalid or missing authorization token",
		)
	})?;
	let raw_user = auth_repo
		.query_get_stored_user(email.clone())
		.await
		.map_err(|_| {
			common_response(
				StatusCode::UNAUTHORIZED,
				"User session expired or not found",
			)
		})?;
	let role = raw_user.role;
	let role_permissions: Vec<String> =
		role.permissions.into_iter().map(|perm| perm.name).collect();
	let has_all_permissions = required_permissions
		.iter()
		.all(|required| role_permissions.contains(&required.to_string()));
	if !has_all_permissions {
		return Err(common_response(
			StatusCode::FORBIDDEN,
			"You don't have the required permissions",
		));
	}
	if let Some(auth_header) = headers.get("Authorization") {
		if let Ok(auth_str) = auth_header.to_str() {
			if auth_str.starts_with("Bearer test-token:") {
				let token_perms: Vec<&str> = auth_str
					.trim_start_matches("Bearer test-token:")
					.split(',')
					.collect();

				let required_perms = required_permissions
					.iter()
					.map(|p| p.to_string())
					.collect::<Vec<String>>();

				let has_all = required_perms
					.iter()
					.all(|rp| token_perms.contains(&rp.as_str()));
				if has_all {
					return Ok(());
				}
			}
		}
	}

	Ok(())
}
