// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The GET request to get the usage data of a usage plan in a specified time interval.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetUsageInput {
    /// <p>The Id of the usage plan associated with the usage data.</p>
    #[doc(hidden)]
    pub usage_plan_id: ::std::option::Option<::std::string::String>,
    /// <p>The Id of the API key associated with the resultant usage data.</p>
    #[doc(hidden)]
    pub key_id: ::std::option::Option<::std::string::String>,
    /// <p>The starting date (e.g., 2016-01-01) of the usage data.</p>
    #[doc(hidden)]
    pub start_date: ::std::option::Option<::std::string::String>,
    /// <p>The ending date (e.g., 2016-12-31) of the usage data.</p>
    #[doc(hidden)]
    pub end_date: ::std::option::Option<::std::string::String>,
    /// <p>The current pagination position in the paged result set.</p>
    #[doc(hidden)]
    pub position: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[doc(hidden)]
    pub limit: ::std::option::Option<i32>,
}
impl GetUsageInput {
    /// <p>The Id of the usage plan associated with the usage data.</p>
    pub fn usage_plan_id(&self) -> ::std::option::Option<&str> {
        self.usage_plan_id.as_deref()
    }
    /// <p>The Id of the API key associated with the resultant usage data.</p>
    pub fn key_id(&self) -> ::std::option::Option<&str> {
        self.key_id.as_deref()
    }
    /// <p>The starting date (e.g., 2016-01-01) of the usage data.</p>
    pub fn start_date(&self) -> ::std::option::Option<&str> {
        self.start_date.as_deref()
    }
    /// <p>The ending date (e.g., 2016-12-31) of the usage data.</p>
    pub fn end_date(&self) -> ::std::option::Option<&str> {
        self.end_date.as_deref()
    }
    /// <p>The current pagination position in the paged result set.</p>
    pub fn position(&self) -> ::std::option::Option<&str> {
        self.position.as_deref()
    }
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    pub fn limit(&self) -> ::std::option::Option<i32> {
        self.limit
    }
}
impl GetUsageInput {
    /// Creates a new builder-style object to manufacture [`GetUsageInput`](crate::operation::get_usage::GetUsageInput).
    pub fn builder() -> crate::operation::get_usage::builders::GetUsageInputBuilder {
        crate::operation::get_usage::builders::GetUsageInputBuilder::default()
    }
}

/// A builder for [`GetUsageInput`](crate::operation::get_usage::GetUsageInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetUsageInputBuilder {
    pub(crate) usage_plan_id: ::std::option::Option<::std::string::String>,
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
    pub(crate) start_date: ::std::option::Option<::std::string::String>,
    pub(crate) end_date: ::std::option::Option<::std::string::String>,
    pub(crate) position: ::std::option::Option<::std::string::String>,
    pub(crate) limit: ::std::option::Option<i32>,
}
impl GetUsageInputBuilder {
    /// <p>The Id of the usage plan associated with the usage data.</p>
    pub fn usage_plan_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.usage_plan_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Id of the usage plan associated with the usage data.</p>
    pub fn set_usage_plan_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.usage_plan_id = input;
        self
    }
    /// <p>The Id of the API key associated with the resultant usage data.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Id of the API key associated with the resultant usage data.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// <p>The starting date (e.g., 2016-01-01) of the usage data.</p>
    pub fn start_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.start_date = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The starting date (e.g., 2016-01-01) of the usage data.</p>
    pub fn set_start_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.start_date = input;
        self
    }
    /// <p>The ending date (e.g., 2016-12-31) of the usage data.</p>
    pub fn end_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.end_date = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ending date (e.g., 2016-12-31) of the usage data.</p>
    pub fn set_end_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.end_date = input;
        self
    }
    /// <p>The current pagination position in the paged result set.</p>
    pub fn position(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.position = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The current pagination position in the paged result set.</p>
    pub fn set_position(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.position = input;
        self
    }
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// Consumes the builder and constructs a [`GetUsageInput`](crate::operation::get_usage::GetUsageInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_usage::GetUsageInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_usage::GetUsageInput {
            usage_plan_id: self.usage_plan_id,
            key_id: self.key_id,
            start_date: self.start_date,
            end_date: self.end_date,
            position: self.position,
            limit: self.limit,
        })
    }
}
