use crate::utils::{ApiRes, ApiResponse, ApiStatuses};

#[catch(404)]
pub fn not_found() -> ApiRes<String> {
    ApiResponse::not_found(String::from(
        "The API route you are looking for does not exist.",
    ))
}

#[catch(500)]
pub fn internal_err() -> ApiRes<String> {
    ApiResponse::error(String::from(
        "Unfortunately, there has been an internal error on our part.",
    ))
}
