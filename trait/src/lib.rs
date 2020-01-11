//! Transform an arbitrary structs to a http query params.

extern crate query_params_serialize;

/// Query params trait.
pub trait QueryParams {
    /// Returns query parameters as string.
    fn query_params(&self) -> query_params_serialize::Result<String>;
}

impl<S: serde::ser::Serialize> QueryParams for S {
    fn query_params(&self) -> query_params_serialize::Result<String> {
        query_params_serialize::to_string(self)
    }
}
