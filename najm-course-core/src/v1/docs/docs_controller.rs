use crate::{
	options::{OptionsCreateRequestDto, OptionsItemDto, OptionsResponseListDto, OptionsUpdateRequestDto}, questions::{QuestionsCreateRequestDto, QuestionsItemDto, QuestionsResponseListDto}, sessions::{SessionsCreateRequestDto, SessionsDetailResponseDto, SessionsResponseDto, SessionsUpdateRequestDto}, tests::{TestsCreateRequestDto, TestsItemDto, TestsResponseListDto, TestsUpdateRequestDto}, v1::{
		auth, options, permissions, questions, roles, sessions, tests, users, AuthLoginRequestDto, AuthLoginResponsetDto, AuthResendOtpRequestDto, AuthVerifyEmailRequestDto
	}, AuthNewPasswordRequestDto, AuthRefreshTokenRequestDto, MessageResponseDto, MetaRequestDto, MetaResponseDto, PermissionsItemDto, PermissionsRequestDto, QuestionsUpdateRequestDto, ResponseListSuccessDto, ResponseSuccessDto, RolesItemDto, RolesRequestCreateDto, RolesRequestUpdateDto, TokenDto, UsersCreateRequestDto, UsersDetailItemDto, UsersItemDto, UsersListItemDto, UsersUpdateRequestDto
};
use utoipa::{
	openapi::security::{Http, HttpAuthScheme, SecurityScheme},
	Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
     auth::auth_controller::post_login,
     auth::auth_controller::post_register,
     auth::auth_controller::post_verify_email,
     auth::auth_controller::post_resend_otp,
     auth::auth_controller::post_refresh_token,
     auth::auth_controller::post_forgot_password,
     auth::auth_controller::post_new_password,
     users::users_controller::post_create_user,
     users::users_controller::put_update_user,
     users::users_controller::put_update_user_me,
     users::users_controller::patch_user_active_status,
     users::users_controller::delete_user,
     users::users_controller::get_user_by_id,
     users::users_controller::get_user_me,
     users::users_controller::get_user_list,
     users::users_controller::put_change_password,
     roles::roles_controller::get_role_list,
     roles::roles_controller::get_role_by_id,
     roles::roles_controller::post_create_role,
     roles::roles_controller::put_update_role,
     roles::roles_controller::delete_role,
     options::options_controller::get_option_list,
     options::options_controller::get_option_by_id,
     options::options_controller::post_create_option,
     options::options_controller::put_update_option,
     options::options_controller::delete_option,
     questions::questions_controller::get_question_list,
     questions::questions_controller::get_question_by_id,
     questions::questions_controller::post_create_question,
     questions::questions_controller::put_update_question,
     questions::questions_controller::delete_question,
     tests::tests_controller::get_test_list,
     tests::tests_controller::get_test_by_id,
     tests::tests_controller::post_create_test,
     tests::tests_controller::put_update_test,
     tests::tests_controller::delete_test,
     sessions::sessions_controller::get_session_list,
     sessions::sessions_controller::get_session_by_id,
     sessions::sessions_controller::post_create_session,
     sessions::sessions_controller::put_update_session,
     sessions::sessions_controller::delete_session,
     permissions::permissions_controller::get_permission_list,
     permissions::permissions_controller::get_permission_by_id,
     permissions::permissions_controller::post_create_permission,
     permissions::permissions_controller::put_update_permission,
     permissions::permissions_controller::delete_permission
    ),
    components(
        schemas(
           MetaRequestDto,
           MetaResponseDto,
           MessageResponseDto,
           AuthLoginRequestDto,
           AuthLoginResponsetDto,
           AuthVerifyEmailRequestDto,
           AuthResendOtpRequestDto,
           AuthNewPasswordRequestDto,
           AuthRefreshTokenRequestDto,
           ResponseSuccessDto<TokenDto>,
           RolesItemDto,
           RolesRequestCreateDto, 
           RolesRequestUpdateDto,
           PermissionsRequestDto,
           PermissionsItemDto,
           UsersItemDto,
           UsersListItemDto,
           UsersUpdateRequestDto,
           UsersCreateRequestDto,
           OptionsItemDto,
           OptionsResponseListDto,
           OptionsCreateRequestDto,
           OptionsUpdateRequestDto,
           QuestionsItemDto,
           QuestionsCreateRequestDto,
           QuestionsUpdateRequestDto,
           QuestionsResponseListDto,
           TestsItemDto,
           TestsCreateRequestDto,
           TestsUpdateRequestDto,
           TestsResponseListDto,
           SessionsCreateRequestDto,
           SessionsResponseDto,
           SessionsDetailResponseDto,
           SessionsUpdateRequestDto,
           TestsCreateRequestDto,
           TestsUpdateRequestDto,
           TestsResponseListDto,
           ResponseSuccessDto<AuthLoginResponsetDto>,
           ResponseListSuccessDto<Vec<RolesItemDto>>,
           ResponseSuccessDto<RolesItemDto>,
           ResponseListSuccessDto<Vec<UsersListItemDto>>,
           ResponseSuccessDto<UsersDetailItemDto>,
           ResponseListSuccessDto<Vec<PermissionsItemDto>>,
           ResponseSuccessDto<PermissionsItemDto>,
           ResponseListSuccessDto<Vec<OptionsResponseListDto>>,
           ResponseSuccessDto<OptionsItemDto>,
           ResponseListSuccessDto<Vec<TestsResponseListDto>>,
           ResponseSuccessDto<TestsItemDto>,
           ResponseListSuccessDto<Vec<QuestionsResponseListDto>>,
           ResponseSuccessDto<QuestionsItemDto>
        )
    ),
    info(
        title = "NAJM Course API",
        description = "NAJM Course API",
        version = "0.1.0",
        contact(
            name = "Maulana Sodiqin",
            url = ""
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (
            name = "Authentication", description = "List of Authentication Endpoints",
        ),
        (
            name = "Users", description = "List of Users Endpoints",
        ),
        (
            name = "Roles", description = "List of Roles Endpoints",
        ),
        (
            name = "Permissions", description = "List of Permissions Endpoints"
        ),
        (
            name = "Tests", description = "List of Tests Endpoints"
        ),
        (
            name = "Sessions", description = "List of Sessions Endpoints"
        ),
        (
            name = "Options", description = "List of Options Endpoints"
        ),
        (
            name = "Questions", description = "List of Questions Endpoints"
        ),
    )
)]

pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		if let Some(components) = openapi.components.as_mut() {
			components.add_security_scheme(
				"Bearer",
				SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
			);
		}
	}
}
