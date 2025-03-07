// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies options related to data preview for viewing a sample of your data.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StreamingDataPreviewOptions {
    /// <p>The polling time in milliseconds.</p>
    #[doc(hidden)]
    pub polling_time: ::std::option::Option<i64>,
    /// <p>The limit to the number of records polled.</p>
    #[doc(hidden)]
    pub record_polling_limit: ::std::option::Option<i64>,
}
impl StreamingDataPreviewOptions {
    /// <p>The polling time in milliseconds.</p>
    pub fn polling_time(&self) -> ::std::option::Option<i64> {
        self.polling_time
    }
    /// <p>The limit to the number of records polled.</p>
    pub fn record_polling_limit(&self) -> ::std::option::Option<i64> {
        self.record_polling_limit
    }
}
impl StreamingDataPreviewOptions {
    /// Creates a new builder-style object to manufacture [`StreamingDataPreviewOptions`](crate::types::StreamingDataPreviewOptions).
    pub fn builder() -> crate::types::builders::StreamingDataPreviewOptionsBuilder {
        crate::types::builders::StreamingDataPreviewOptionsBuilder::default()
    }
}

/// A builder for [`StreamingDataPreviewOptions`](crate::types::StreamingDataPreviewOptions).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StreamingDataPreviewOptionsBuilder {
    pub(crate) polling_time: ::std::option::Option<i64>,
    pub(crate) record_polling_limit: ::std::option::Option<i64>,
}
impl StreamingDataPreviewOptionsBuilder {
    /// <p>The polling time in milliseconds.</p>
    pub fn polling_time(mut self, input: i64) -> Self {
        self.polling_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The polling time in milliseconds.</p>
    pub fn set_polling_time(mut self, input: ::std::option::Option<i64>) -> Self {
        self.polling_time = input;
        self
    }
    /// <p>The limit to the number of records polled.</p>
    pub fn record_polling_limit(mut self, input: i64) -> Self {
        self.record_polling_limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The limit to the number of records polled.</p>
    pub fn set_record_polling_limit(mut self, input: ::std::option::Option<i64>) -> Self {
        self.record_polling_limit = input;
        self
    }
    /// Consumes the builder and constructs a [`StreamingDataPreviewOptions`](crate::types::StreamingDataPreviewOptions).
    pub fn build(self) -> crate::types::StreamingDataPreviewOptions {
        crate::types::StreamingDataPreviewOptions {
            polling_time: self.polling_time,
            record_polling_limit: self.record_polling_limit,
        }
    }
}
