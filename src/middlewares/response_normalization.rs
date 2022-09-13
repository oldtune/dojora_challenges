use std::{
    future::{ready, Ready},
    task::Poll,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    error::Error,
    web::Bytes,
};
use futures_util::future::LocalBoxFuture;

pub struct ResponseNormalizationMiddlewareFactory;
pub struct ResponseNormalizationMiddleware<S> {
    service: S,
}

impl<S, B> Transform<S, ServiceRequest> for ResponseNormalizationMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + std::fmt::Debug + actix_web::body::MessageBody,
{
    type Response = ServiceResponse<Bytes>;

    type Error = Error;

    type Transform = ResponseNormalizationMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ResponseNormalizationMiddleware { service }))
    }
}

impl<S, B> Service<ServiceRequest> for ResponseNormalizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static + std::fmt::Debug + actix_web::body::MessageBody,
{
    type Response = ServiceResponse<Bytes>;

    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &self,
        ctx: &mut core::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let fut = self.service.call(req);
        Box::pin(async {
            let res = fut.await?;

            let (req, res) = res.into_parts();
            let (res, body) = res.into_parts();
            let body_bytes = actix_web::body::to_bytes(body).await.ok().unwrap();
            let res = res.set_body(body_bytes);
            let res = ServiceResponse::new(req, res);

            Ok(res)
        })
    }
}
