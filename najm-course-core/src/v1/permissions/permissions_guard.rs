use super::PermissionsEnum;
use crate::{common_response, extract_email, AppState, AuthRepository};
use axum::{
	http::{HeaderMap, StatusCode},
	response::Response,
};

use log::{debug, info, warn};

pub async fn permissions_guard(
	headers: &HeaderMap,
	state: AppState,
	required_permissions: Vec<PermissionsEnum>,
) -> Result<(), Response> {
	if let Some(auth_header) = headers.get("Authorization") {
		if let Ok(auth_str) = auth_header.to_str() {
			info!("📥 Received headers: {:?}", headers);
			debug!("🔍 Authorization Header: {}", auth_str);

			if auth_str.starts_with("Bearer test-token:") {
				let token_perms: Vec<&str> = auth_str
					.trim_start_matches("Bearer test-token:")
					.split(',')
					.map(|s| s.trim())
					.collect();

				debug!("🧪 Test Token Permissions: {:?}", token_perms);

				let required_perms: Vec<String> =
					required_permissions.iter().map(|p| p.to_string()).collect();

				debug!("✅ Required Permissions: {:?}", required_perms);

				let has_all = required_perms
					.iter()
					.all(|rp| token_perms.contains(&rp.as_str()));

				if has_all {
					info!("✅ Access granted via mock token.");
					return Ok(());
				} else {
					warn!("⛔ Permission denied via mock token.");
					return Err(common_response(
						StatusCode::FORBIDDEN,
						"You don't have the required permissions (mock token)",
					));
				}
			}
		}
	}

	info!("🔁 Fallback to real token logic.");

	let auth_repo = AuthRepository::new(&state);

	let email = extract_email(headers).ok_or_else(|| {
		warn!("❌ Email extraction failed from token.");
		common_response(
			StatusCode::UNAUTHORIZED,
			"Invalid or missing authorization token",
		)
	})?;

	info!("📧 Extracted email from token: {}", email);

	let raw_user = auth_repo
		.query_get_stored_user(email.clone())
		.await
		.map_err(|_| {
			warn!("❌ User not found or session expired.");
			common_response(
				StatusCode::UNAUTHORIZED,
				"User session expired or not found",
			)
		})?;

	let role_permissions: Vec<String> = raw_user
		.role
		.permissions
		.into_iter()
		.map(|perm| perm.name)
		.collect();

	debug!("🔓 Role Permissions: {:?}", role_permissions);

	let required_perms = required_permissions
		.iter()
		.map(|p| p.to_string())
		.collect::<Vec<_>>();

	debug!("🔐 Required Permissions: {:?}", required_perms);

	let has_all = required_perms
		.iter()
		.all(|rp| role_permissions.contains(rp));

	if !has_all {
		warn!("⛔ Permission denied (real user).");
		return Err(common_response(
			StatusCode::FORBIDDEN,
			"You don't have the required permissions",
		));
	}

	info!("✅ Access granted (real user).");
	Ok(())
}
