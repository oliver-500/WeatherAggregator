use actix_web::{
    dev::ServiceResponse,
    http::{header},
    middleware::{ErrorHandlerResponse},
    Result,
};
use actix_web::body::MessageBody;
use actix_web::http::StatusCode;
use chrono::Utc;
use crate::org::unibl::etf::controllers::errors::{GenericServiceError, GenericServiceErrorDetails};
use crate::org::unibl::etf::model::errors::adapter_error::AdapterError;

pub fn internal_error_handler<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>>
where
    B: MessageBody + 'static,
{
    let (req, res) = res.into_parts();

    // 1. Check if the body already exists and is likely our JSON error
    // We check if Content-Type is already application/json
    let has_json_body = res.headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(|v| v.contains("application/json"))
        .unwrap_or(false);

    if has_json_body {
        // If it already has a JSON body from to_response(),
        // just wrap it back up and return it as is.
        let res = ServiceResponse::new(req, res)
            .map_into_boxed_body()
            .map_into_right_body();
        return Ok(ErrorHandlerResponse::Response(res));
    }

    let status = res.status();

    if status == StatusCode::BAD_REQUEST {
        let raw_error_msg = res.error().map(|e| e.to_string());

        if let Some(msg) = raw_error_msg {
            let error_message = GenericServiceError {
                error: GenericServiceErrorDetails {
                    code: AdapterError::ValidationError ,
                    code_numeric: status.as_u16(),
                    message: msg,
                    provider: "".to_string(),
                    timestamp: Utc::now(),
                }
            };
            println!("da");

            let body = serde_json::to_string(&error_message)
                .unwrap_or_else(|_| "{\"error\":\"Critical error\"}".to_string());
            
            let mut new_res = res.set_body(body);
            new_res.headers_mut().insert(
                header::CONTENT_TYPE,
                header::HeaderValue::from_static("application/json"),
            );

            let res = ServiceResponse::new(req, new_res)
                .map_into_boxed_body()
                .map_into_right_body();

            return Ok(ErrorHandlerResponse::Response(res));
        };

    }
    else if  status == StatusCode::NOT_FOUND {
        let error_message = GenericServiceError {
            error: GenericServiceErrorDetails {
                code: AdapterError::EndpointNotFound,
                code_numeric: 400,
                message: "Requested url not found.".to_string(),
                provider: "".to_string(),
                timestamp: Utc::now(),
            }
        };

        let body = serde_json::to_string(&error_message)
            .unwrap_or_else(|_| "{\"error\":\"Critical error\"}".to_string());

        let mut new_res = res.set_body(body);
        new_res.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let res = ServiceResponse::new(req, new_res)
            .map_into_boxed_body()
            .map_into_right_body();

        return Ok(ErrorHandlerResponse::Response(res));

    }

    println!("fallback error in error handler");
    // 2. Fallback: Create the generic error if no specific error was passed up
    let error_message = GenericServiceError {
        error: GenericServiceErrorDetails {
            code: AdapterError::ServerError(None),
            code_numeric: 500,
            message: "An internal server error occurred. Please try again later.".to_string(),
            provider: "".to_string(),
            timestamp: Utc::now(),
        }
    };

    let body = serde_json::to_string(&error_message)
        .unwrap_or_else(|_| "{\"message\":\"Error\"}".to_string());

    let mut res = res.set_body(body);
    res.headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json")
    );

    let res = ServiceResponse::new(req, res)
        .map_into_boxed_body()
        .map_into_right_body();

    Ok(ErrorHandlerResponse::Response(res))

}