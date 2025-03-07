// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopWorkspacesOutput {
    /// <p>Information about the WorkSpaces that could not be stopped.</p>
    #[doc(hidden)]
    pub failed_requests:
        ::std::option::Option<::std::vec::Vec<crate::types::FailedWorkspaceChangeRequest>>,
    _request_id: Option<String>,
}
impl StopWorkspacesOutput {
    /// <p>Information about the WorkSpaces that could not be stopped.</p>
    pub fn failed_requests(
        &self,
    ) -> ::std::option::Option<&[crate::types::FailedWorkspaceChangeRequest]> {
        self.failed_requests.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for StopWorkspacesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StopWorkspacesOutput {
    /// Creates a new builder-style object to manufacture [`StopWorkspacesOutput`](crate::operation::stop_workspaces::StopWorkspacesOutput).
    pub fn builder() -> crate::operation::stop_workspaces::builders::StopWorkspacesOutputBuilder {
        crate::operation::stop_workspaces::builders::StopWorkspacesOutputBuilder::default()
    }
}

/// A builder for [`StopWorkspacesOutput`](crate::operation::stop_workspaces::StopWorkspacesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StopWorkspacesOutputBuilder {
    pub(crate) failed_requests:
        ::std::option::Option<::std::vec::Vec<crate::types::FailedWorkspaceChangeRequest>>,
    _request_id: Option<String>,
}
impl StopWorkspacesOutputBuilder {
    /// Appends an item to `failed_requests`.
    ///
    /// To override the contents of this collection use [`set_failed_requests`](Self::set_failed_requests).
    ///
    /// <p>Information about the WorkSpaces that could not be stopped.</p>
    pub fn failed_requests(mut self, input: crate::types::FailedWorkspaceChangeRequest) -> Self {
        let mut v = self.failed_requests.unwrap_or_default();
        v.push(input);
        self.failed_requests = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the WorkSpaces that could not be stopped.</p>
    pub fn set_failed_requests(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FailedWorkspaceChangeRequest>>,
    ) -> Self {
        self.failed_requests = input;
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
    /// Consumes the builder and constructs a [`StopWorkspacesOutput`](crate::operation::stop_workspaces::StopWorkspacesOutput).
    pub fn build(self) -> crate::operation::stop_workspaces::StopWorkspacesOutput {
        crate::operation::stop_workspaces::StopWorkspacesOutput {
            failed_requests: self.failed_requests,
            _request_id: self._request_id,
        }
    }
}
