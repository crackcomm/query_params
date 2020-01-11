//! Transform an arbitrary structs to a http query params.

/// Query params trait.
pub trait QueryParams {
    /// Returns query parameters as string.
    fn query_params(&self) -> String;
}
