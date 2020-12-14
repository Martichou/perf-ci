use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{
    bearer::{BearerAuth, Config},
    AuthenticationError,
};

fn validate_token(token: &str) -> bool {
	if token == std::env::var("API_KEY").expect("API_KEY must be set") {
		true
	} else {
		false
	}
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
	if validate_token(credentials.token()) {
		Ok(req)
	} else {
		let config = req
			.app_data::<Config>()
			.cloned()
			.unwrap_or_else(Default::default);
		Err(AuthenticationError::from(config).into())
	}
}