use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::Method,
    middleware::Next,
    Error,
};

pub async fn not_found(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // pre-processing
    if req.method() == Method::POST {
        log::debug!("This is a post request, {}", req.path());
    }
    next.call(req).await
    // post-processing
}
