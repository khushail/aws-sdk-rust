// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchExecuteStatementOutput {
    /// <p>The response to each PartiQL statement in the batch.</p>
    #[doc(hidden)]
    pub responses: ::std::option::Option<::std::vec::Vec<crate::types::BatchStatementResponse>>,
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    #[doc(hidden)]
    pub consumed_capacity: ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>,
    _request_id: Option<String>,
}
impl BatchExecuteStatementOutput {
    /// <p>The response to each PartiQL statement in the batch.</p>
    pub fn responses(&self) -> ::std::option::Option<&[crate::types::BatchStatementResponse]> {
        self.responses.as_deref()
    }
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    pub fn consumed_capacity(&self) -> ::std::option::Option<&[crate::types::ConsumedCapacity]> {
        self.consumed_capacity.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchExecuteStatementOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchExecuteStatementOutput {
    /// Creates a new builder-style object to manufacture [`BatchExecuteStatementOutput`](crate::operation::batch_execute_statement::BatchExecuteStatementOutput).
    pub fn builder(
    ) -> crate::operation::batch_execute_statement::builders::BatchExecuteStatementOutputBuilder
    {
        crate::operation::batch_execute_statement::builders::BatchExecuteStatementOutputBuilder::default()
    }
}

/// A builder for [`BatchExecuteStatementOutput`](crate::operation::batch_execute_statement::BatchExecuteStatementOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchExecuteStatementOutputBuilder {
    pub(crate) responses:
        ::std::option::Option<::std::vec::Vec<crate::types::BatchStatementResponse>>,
    pub(crate) consumed_capacity:
        ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>,
    _request_id: Option<String>,
}
impl BatchExecuteStatementOutputBuilder {
    /// Appends an item to `responses`.
    ///
    /// To override the contents of this collection use [`set_responses`](Self::set_responses).
    ///
    /// <p>The response to each PartiQL statement in the batch.</p>
    pub fn responses(mut self, input: crate::types::BatchStatementResponse) -> Self {
        let mut v = self.responses.unwrap_or_default();
        v.push(input);
        self.responses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The response to each PartiQL statement in the batch.</p>
    pub fn set_responses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BatchStatementResponse>>,
    ) -> Self {
        self.responses = input;
        self
    }
    /// Appends an item to `consumed_capacity`.
    ///
    /// To override the contents of this collection use [`set_consumed_capacity`](Self::set_consumed_capacity).
    ///
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    pub fn consumed_capacity(mut self, input: crate::types::ConsumedCapacity) -> Self {
        let mut v = self.consumed_capacity.unwrap_or_default();
        v.push(input);
        self.consumed_capacity = ::std::option::Option::Some(v);
        self
    }
    /// <p>The capacity units consumed by the entire operation. The values of the list are ordered according to the ordering of the statements.</p>
    pub fn set_consumed_capacity(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ConsumedCapacity>>,
    ) -> Self {
        self.consumed_capacity = input;
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
    /// Consumes the builder and constructs a [`BatchExecuteStatementOutput`](crate::operation::batch_execute_statement::BatchExecuteStatementOutput).
    pub fn build(self) -> crate::operation::batch_execute_statement::BatchExecuteStatementOutput {
        crate::operation::batch_execute_statement::BatchExecuteStatementOutput {
            responses: self.responses,
            consumed_capacity: self.consumed_capacity,
            _request_id: self._request_id,
        }
    }
}
