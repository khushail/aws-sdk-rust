// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubmitMultiRegionAccessPointRoutesOutput {
    _request_id: Option<String>,
}
impl ::aws_http::request_id::RequestId for SubmitMultiRegionAccessPointRoutesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SubmitMultiRegionAccessPointRoutesOutput {
    /// Creates a new builder-style object to manufacture [`SubmitMultiRegionAccessPointRoutesOutput`](crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput).
    pub fn builder() -> crate::operation::submit_multi_region_access_point_routes::builders::SubmitMultiRegionAccessPointRoutesOutputBuilder{
        crate::operation::submit_multi_region_access_point_routes::builders::SubmitMultiRegionAccessPointRoutesOutputBuilder::default()
    }
}

/// A builder for [`SubmitMultiRegionAccessPointRoutesOutput`](crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SubmitMultiRegionAccessPointRoutesOutputBuilder {
    _request_id: Option<String>,
}
impl SubmitMultiRegionAccessPointRoutesOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`SubmitMultiRegionAccessPointRoutesOutput`](crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput).
    pub fn build(self) -> crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput{
        crate::operation::submit_multi_region_access_point_routes::SubmitMultiRegionAccessPointRoutesOutput {
            _request_id: self._request_id,
        }
    }
}
