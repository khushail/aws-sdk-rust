// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateQuantumTaskOutput {
    /// <p>The ARN of the task created by the request.</p>
    #[doc(hidden)]
    pub quantum_task_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateQuantumTaskOutput {
    /// <p>The ARN of the task created by the request.</p>
    pub fn quantum_task_arn(&self) -> ::std::option::Option<&str> {
        self.quantum_task_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateQuantumTaskOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateQuantumTaskOutput {
    /// Creates a new builder-style object to manufacture [`CreateQuantumTaskOutput`](crate::operation::create_quantum_task::CreateQuantumTaskOutput).
    pub fn builder(
    ) -> crate::operation::create_quantum_task::builders::CreateQuantumTaskOutputBuilder {
        crate::operation::create_quantum_task::builders::CreateQuantumTaskOutputBuilder::default()
    }
}

/// A builder for [`CreateQuantumTaskOutput`](crate::operation::create_quantum_task::CreateQuantumTaskOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateQuantumTaskOutputBuilder {
    pub(crate) quantum_task_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateQuantumTaskOutputBuilder {
    /// <p>The ARN of the task created by the request.</p>
    pub fn quantum_task_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.quantum_task_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the task created by the request.</p>
    pub fn set_quantum_task_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.quantum_task_arn = input;
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
    /// Consumes the builder and constructs a [`CreateQuantumTaskOutput`](crate::operation::create_quantum_task::CreateQuantumTaskOutput).
    pub fn build(self) -> crate::operation::create_quantum_task::CreateQuantumTaskOutput {
        crate::operation::create_quantum_task::CreateQuantumTaskOutput {
            quantum_task_arn: self.quantum_task_arn,
            _request_id: self._request_id,
        }
    }
}
