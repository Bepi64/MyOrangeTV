use std::collections::HashMap;

#[derive(serde::Deserialize, Debug)]
pub struct ApiResponse
{
    result: ApiResult,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApiResult
{
    responseCode: String,
    message: String,
    data: HashMap<String, String>,
}

