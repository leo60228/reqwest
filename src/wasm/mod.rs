use wasm_bindgen::JsCast;

mod body;
mod client;
pub(crate) mod error;
mod request;
mod response;

pub use self::body::Body;
pub use self::client::{Client, ClientBuilder};
pub use self::request::{Request, RequestBuilder};
pub use self::response::Response;


async fn promise<T>(promise: js_sys::Promise) -> crate::Result<T>
where
    T: JsCast,
{
    use wasm_bindgen_futures::futures_0_3::JsFuture;

    let js_val = JsFuture::from(promise)
        .await
        .map_err(crate::error::from)?;

    // TODO: is unchecked_into unsafe?
    let t = js_val.dyn_into::<T>().unwrap();
    Ok(t)
}
