// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Result structure used for requests to updated project configuration. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateProjectOutput {
    /// <p> Detailed information about the updated AWS Mobile Hub project. </p>
    #[doc(hidden)]
    pub details: ::std::option::Option<crate::types::ProjectDetails>,
    _request_id: Option<String>,
}
impl UpdateProjectOutput {
    /// <p> Detailed information about the updated AWS Mobile Hub project. </p>
    pub fn details(&self) -> ::std::option::Option<&crate::types::ProjectDetails> {
        self.details.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateProjectOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateProjectOutput {
    /// Creates a new builder-style object to manufacture [`UpdateProjectOutput`](crate::operation::update_project::UpdateProjectOutput).
    pub fn builder() -> crate::operation::update_project::builders::UpdateProjectOutputBuilder {
        crate::operation::update_project::builders::UpdateProjectOutputBuilder::default()
    }
}

/// A builder for [`UpdateProjectOutput`](crate::operation::update_project::UpdateProjectOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateProjectOutputBuilder {
    pub(crate) details: ::std::option::Option<crate::types::ProjectDetails>,
    _request_id: Option<String>,
}
impl UpdateProjectOutputBuilder {
    /// <p> Detailed information about the updated AWS Mobile Hub project. </p>
    pub fn details(mut self, input: crate::types::ProjectDetails) -> Self {
        self.details = ::std::option::Option::Some(input);
        self
    }
    /// <p> Detailed information about the updated AWS Mobile Hub project. </p>
    pub fn set_details(
        mut self,
        input: ::std::option::Option<crate::types::ProjectDetails>,
    ) -> Self {
        self.details = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`UpdateProjectOutput`](crate::operation::update_project::UpdateProjectOutput).
    pub fn build(self) -> crate::operation::update_project::UpdateProjectOutput {
        crate::operation::update_project::UpdateProjectOutput {
            details: self.details,
            _request_id: self._request_id,
        }
    }
}
