// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetStreamKeyOutput {
    /// <p></p>
    #[doc(hidden)]
    pub stream_keys: ::std::option::Option<::std::vec::Vec<crate::types::StreamKey>>,
    /// <p></p>
    #[doc(hidden)]
    pub errors: ::std::option::Option<::std::vec::Vec<crate::types::BatchError>>,
    _request_id: Option<String>,
}
impl BatchGetStreamKeyOutput {
    /// <p></p>
    pub fn stream_keys(&self) -> ::std::option::Option<&[crate::types::StreamKey]> {
        self.stream_keys.as_deref()
    }
    /// <p></p>
    pub fn errors(&self) -> ::std::option::Option<&[crate::types::BatchError]> {
        self.errors.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for BatchGetStreamKeyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl BatchGetStreamKeyOutput {
    /// Creates a new builder-style object to manufacture [`BatchGetStreamKeyOutput`](crate::operation::batch_get_stream_key::BatchGetStreamKeyOutput).
    pub fn builder(
    ) -> crate::operation::batch_get_stream_key::builders::BatchGetStreamKeyOutputBuilder {
        crate::operation::batch_get_stream_key::builders::BatchGetStreamKeyOutputBuilder::default()
    }
}

/// A builder for [`BatchGetStreamKeyOutput`](crate::operation::batch_get_stream_key::BatchGetStreamKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetStreamKeyOutputBuilder {
    pub(crate) stream_keys: ::std::option::Option<::std::vec::Vec<crate::types::StreamKey>>,
    pub(crate) errors: ::std::option::Option<::std::vec::Vec<crate::types::BatchError>>,
    _request_id: Option<String>,
}
impl BatchGetStreamKeyOutputBuilder {
    /// Appends an item to `stream_keys`.
    ///
    /// To override the contents of this collection use [`set_stream_keys`](Self::set_stream_keys).
    ///
    /// <p></p>
    pub fn stream_keys(mut self, input: crate::types::StreamKey) -> Self {
        let mut v = self.stream_keys.unwrap_or_default();
        v.push(input);
        self.stream_keys = ::std::option::Option::Some(v);
        self
    }
    /// <p></p>
    pub fn set_stream_keys(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StreamKey>>,
    ) -> Self {
        self.stream_keys = input;
        self
    }
    /// Appends an item to `errors`.
    ///
    /// To override the contents of this collection use [`set_errors`](Self::set_errors).
    ///
    /// <p></p>
    pub fn errors(mut self, input: crate::types::BatchError) -> Self {
        let mut v = self.errors.unwrap_or_default();
        v.push(input);
        self.errors = ::std::option::Option::Some(v);
        self
    }
    /// <p></p>
    pub fn set_errors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BatchError>>,
    ) -> Self {
        self.errors = input;
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
    /// Consumes the builder and constructs a [`BatchGetStreamKeyOutput`](crate::operation::batch_get_stream_key::BatchGetStreamKeyOutput).
    pub fn build(self) -> crate::operation::batch_get_stream_key::BatchGetStreamKeyOutput {
        crate::operation::batch_get_stream_key::BatchGetStreamKeyOutput {
            stream_keys: self.stream_keys,
            errors: self.errors,
            _request_id: self._request_id,
        }
    }
}
