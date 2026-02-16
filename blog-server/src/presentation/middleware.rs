use std::future::{ready, Ready};

use actix_web::{dev::{Service, ServiceRequest, ServiceResponse, Transform}, Error, HttpMessage};
use actix_web::dev::forward_ready;
use actix_web_lab::__reexports::futures_util::future::LocalBoxFuture;
use crate::domain::jwt::AccessClaims;
use crate::infrastructure::jwt::decode_token;

pub struct AutMiddleware;

impl <S, B> Transform<S, ServiceRequest> for AutMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AutMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AutMiddlewareService { service }))
    }
}

pub struct AutMiddlewareService<S>{
    service: S,
}

impl <S, B> Service<ServiceRequest> for AutMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.method() == actix_web::http::Method::GET && req.path() == "/api/posts" {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        let auth_token = req.headers()
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .and_then(|token| token.strip_prefix("Bearer "));

        match auth_token {
            Some(token) => {
                match decode_token::<AccessClaims>(token.to_string()) {
                    Ok(token) => {
                        req.extensions_mut().insert(token);
                        let fut = self.service.call(req);
                        Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        })
                    },
                    Err(error) => {
                        Box::pin(
                        async {
                            Err(actix_web::error::ErrorUnauthorized("Token expired or invalid"))
                        }
                    )
                    }
                }
            }
            None => {
                Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Authorization header missing"))
                })
            }
        }
    }
}
