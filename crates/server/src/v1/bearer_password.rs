use std::{future::Future, pin::Pin};

use actix_web::FromRequest;

pub struct BearerPassword;

impl FromRequest for BearerPassword {
    type Error = super::ApiError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let authorization_header = req.headers().get("Authorization").cloned();
        let password = req
            .app_data::<super::AppData>()
            .map(|ad| ad.password.clone());

        Box::pin(async move {
            match (dbg!(password), dbg!(authorization_header)) {
                (Some(password), Some(header)) => {
                    let request_password = header.to_str().unwrap_or_default();

                    if request_password != password {
                        return Err(super::ApiError::Unauthorized);
                    }

                    Ok(BearerPassword {})
                }
                _ => Err(super::ApiError::Unauthorized),
            }
        })
    }
}
