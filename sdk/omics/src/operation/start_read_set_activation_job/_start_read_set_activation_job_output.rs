// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartReadSetActivationJobOutput {
    /// <p>The job's ID.</p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The read set's sequence store ID.</p>
    #[doc(hidden)]
    pub sequence_store_id: ::std::option::Option<::std::string::String>,
    /// <p>The job's status.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ReadSetActivationJobStatus>,
    /// <p>When the job was created.</p>
    #[doc(hidden)]
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl StartReadSetActivationJobOutput {
    /// <p>The job's ID.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The read set's sequence store ID.</p>
    pub fn sequence_store_id(&self) -> ::std::option::Option<&str> {
        self.sequence_store_id.as_deref()
    }
    /// <p>The job's status.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ReadSetActivationJobStatus> {
        self.status.as_ref()
    }
    /// <p>When the job was created.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for StartReadSetActivationJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartReadSetActivationJobOutput {
    /// Creates a new builder-style object to manufacture [`StartReadSetActivationJobOutput`](crate::operation::start_read_set_activation_job::StartReadSetActivationJobOutput).
    pub fn builder() -> crate::operation::start_read_set_activation_job::builders::StartReadSetActivationJobOutputBuilder{
        crate::operation::start_read_set_activation_job::builders::StartReadSetActivationJobOutputBuilder::default()
    }
}

/// A builder for [`StartReadSetActivationJobOutput`](crate::operation::start_read_set_activation_job::StartReadSetActivationJobOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StartReadSetActivationJobOutputBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) sequence_store_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::ReadSetActivationJobStatus>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl StartReadSetActivationJobOutputBuilder {
    /// <p>The job's ID.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The job's ID.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The read set's sequence store ID.</p>
    pub fn sequence_store_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.sequence_store_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The read set's sequence store ID.</p>
    pub fn set_sequence_store_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.sequence_store_id = input;
        self
    }
    /// <p>The job's status.</p>
    pub fn status(mut self, input: crate::types::ReadSetActivationJobStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The job's status.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::ReadSetActivationJobStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>When the job was created.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>When the job was created.</p>
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.creation_time = input;
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
    /// Consumes the builder and constructs a [`StartReadSetActivationJobOutput`](crate::operation::start_read_set_activation_job::StartReadSetActivationJobOutput).
    pub fn build(
        self,
    ) -> crate::operation::start_read_set_activation_job::StartReadSetActivationJobOutput {
        crate::operation::start_read_set_activation_job::StartReadSetActivationJobOutput {
            id: self.id,
            sequence_store_id: self.sequence_store_id,
            status: self.status,
            creation_time: self.creation_time,
            _request_id: self._request_id,
        }
    }
}
