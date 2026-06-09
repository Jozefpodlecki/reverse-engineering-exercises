use js_sys::Uint8Array;
use log::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestCache, RequestInit, RequestMode, Response, Window};

use crate::models::{AppError};

#[cfg(debug_assertions)]
const CACHE_MODE: RequestCache = RequestCache::NoStore;

#[cfg(not(debug_assertions))]
const CACHE_MODE: RequestCache = RequestCache::Default;

#[derive(Debug, Clone, PartialEq)]
pub struct ApiClient(Window);

impl ApiClient {
    pub fn new(window: Window) -> Self {
        Self(window)
    }

    // pub async fn get_social(&self) -> Result<Social, AppError> {
    //     let url = "public/social.msgpack";

    //     let headers = Headers::new()?;
    //     // headers.set("User-Agent", "porfolio-wasm")?;
    //     headers.set("X-App-Version", "1.0")?;
    //     headers.set("X-Client", "porfolio-wasm")?;

    //     let request_options = RequestInit::new();
    //     request_options.set_method("GET");
    //     request_options.set_mode(RequestMode::Cors);
    //     request_options.set_cache(CACHE_MODE);
    //     request_options.set_headers(&headers);

    //     let request = Request::new_with_str_and_init(url, &request_options)
    //         .map_err(AppError::failed_to_build_request)?;

    //     let response_value = JsFuture::from(self.0.fetch_with_request(&request))
    //         .await
    //         .map_err(AppError::network_request_failed)?;

    //     let response: Response = response_value.dyn_into()
    //         .map_err(AppError::invalid_response)?;

    //     let buffer = JsFuture::from(response.array_buffer()
    //         .map_err(AppError::failed_to_read_body)?)
    //         .await
    //         .map_err(AppError::failed_to_read_body)?;

    //     let uint8_array = Uint8Array::new(&buffer);
    //     let mut bytes = vec![0; uint8_array.length() as usize];
    //     uint8_array.copy_to(&mut bytes);

    //     let data: Social = rmp_serde::from_slice(&bytes)?;

    //     Ok(data)
    // }
}