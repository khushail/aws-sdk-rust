// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeQueryInput {
    /// <p>The ARN (or the ID suffix of the ARN) of an event data store on which the specified query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by DescribeQueryRequest")]
    #[doc(hidden)]
    pub event_data_store: ::std::option::Option<::std::string::String>,
    /// <p>The query ID.</p>
    #[doc(hidden)]
    pub query_id: ::std::option::Option<::std::string::String>,
}
impl DescribeQueryInput {
    /// <p>The ARN (or the ID suffix of the ARN) of an event data store on which the specified query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by DescribeQueryRequest")]
    pub fn event_data_store(&self) -> ::std::option::Option<&str> {
        self.event_data_store.as_deref()
    }
    /// <p>The query ID.</p>
    pub fn query_id(&self) -> ::std::option::Option<&str> {
        self.query_id.as_deref()
    }
}
impl DescribeQueryInput {
    /// Creates a new builder-style object to manufacture [`DescribeQueryInput`](crate::operation::describe_query::DescribeQueryInput).
    pub fn builder() -> crate::operation::describe_query::builders::DescribeQueryInputBuilder {
        crate::operation::describe_query::builders::DescribeQueryInputBuilder::default()
    }
}

/// A builder for [`DescribeQueryInput`](crate::operation::describe_query::DescribeQueryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeQueryInputBuilder {
    pub(crate) event_data_store: ::std::option::Option<::std::string::String>,
    pub(crate) query_id: ::std::option::Option<::std::string::String>,
}
impl DescribeQueryInputBuilder {
    /// <p>The ARN (or the ID suffix of the ARN) of an event data store on which the specified query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by DescribeQueryRequest")]
    pub fn event_data_store(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.event_data_store = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN (or the ID suffix of the ARN) of an event data store on which the specified query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by DescribeQueryRequest")]
    pub fn set_event_data_store(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.event_data_store = input;
        self
    }
    /// <p>The query ID.</p>
    pub fn query_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.query_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The query ID.</p>
    pub fn set_query_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.query_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeQueryInput`](crate::operation::describe_query::DescribeQueryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_query::DescribeQueryInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_query::DescribeQueryInput {
            event_data_store: self.event_data_store,
            query_id: self.query_id,
        })
    }
}
