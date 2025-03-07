// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ListChannelMessagesInput {
    /// <p>The ARN of the channel.</p>
    #[doc(hidden)]
    pub channel_arn: ::std::option::Option<::std::string::String>,
    /// <p>The order in which you want messages sorted. Default is Descending, based on time created.</p>
    #[doc(hidden)]
    pub sort_order: ::std::option::Option<crate::types::SortOrder>,
    /// <p>The initial or starting time stamp for your requested messages.</p>
    #[doc(hidden)]
    pub not_before: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The final or ending time stamp for your requested messages.</p>
    #[doc(hidden)]
    pub not_after: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The maximum number of messages that you want returned.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token passed by previous API calls until all requested messages are returned.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    #[doc(hidden)]
    pub chime_bearer: ::std::option::Option<::std::string::String>,
}
impl ListChannelMessagesInput {
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(&self) -> ::std::option::Option<&str> {
        self.channel_arn.as_deref()
    }
    /// <p>The order in which you want messages sorted. Default is Descending, based on time created.</p>
    pub fn sort_order(&self) -> ::std::option::Option<&crate::types::SortOrder> {
        self.sort_order.as_ref()
    }
    /// <p>The initial or starting time stamp for your requested messages.</p>
    pub fn not_before(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.not_before.as_ref()
    }
    /// <p>The final or ending time stamp for your requested messages.</p>
    pub fn not_after(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.not_after.as_ref()
    }
    /// <p>The maximum number of messages that you want returned.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token passed by previous API calls until all requested messages are returned.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn chime_bearer(&self) -> ::std::option::Option<&str> {
        self.chime_bearer.as_deref()
    }
}
impl ::std::fmt::Debug for ListChannelMessagesInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListChannelMessagesInput");
        formatter.field("channel_arn", &self.channel_arn);
        formatter.field("sort_order", &self.sort_order);
        formatter.field("not_before", &self.not_before);
        formatter.field("not_after", &self.not_after);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.field("chime_bearer", &self.chime_bearer);
        formatter.finish()
    }
}
impl ListChannelMessagesInput {
    /// Creates a new builder-style object to manufacture [`ListChannelMessagesInput`](crate::operation::list_channel_messages::ListChannelMessagesInput).
    pub fn builder(
    ) -> crate::operation::list_channel_messages::builders::ListChannelMessagesInputBuilder {
        crate::operation::list_channel_messages::builders::ListChannelMessagesInputBuilder::default(
        )
    }
}

/// A builder for [`ListChannelMessagesInput`](crate::operation::list_channel_messages::ListChannelMessagesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ListChannelMessagesInputBuilder {
    pub(crate) channel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) sort_order: ::std::option::Option<crate::types::SortOrder>,
    pub(crate) not_before: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) not_after: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) chime_bearer: ::std::option::Option<::std::string::String>,
}
impl ListChannelMessagesInputBuilder {
    /// <p>The ARN of the channel.</p>
    pub fn channel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the channel.</p>
    pub fn set_channel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_arn = input;
        self
    }
    /// <p>The order in which you want messages sorted. Default is Descending, based on time created.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.sort_order = ::std::option::Option::Some(input);
        self
    }
    /// <p>The order in which you want messages sorted. Default is Descending, based on time created.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.sort_order = input;
        self
    }
    /// <p>The initial or starting time stamp for your requested messages.</p>
    pub fn not_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.not_before = ::std::option::Option::Some(input);
        self
    }
    /// <p>The initial or starting time stamp for your requested messages.</p>
    pub fn set_not_before(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.not_before = input;
        self
    }
    /// <p>The final or ending time stamp for your requested messages.</p>
    pub fn not_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.not_after = ::std::option::Option::Some(input);
        self
    }
    /// <p>The final or ending time stamp for your requested messages.</p>
    pub fn set_not_after(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.not_after = input;
        self
    }
    /// <p>The maximum number of messages that you want returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of messages that you want returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token passed by previous API calls until all requested messages are returned.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token passed by previous API calls until all requested messages are returned.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn chime_bearer(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.chime_bearer = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>AppInstanceUserArn</code> of the user that makes the API call.</p>
    pub fn set_chime_bearer(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.chime_bearer = input;
        self
    }
    /// Consumes the builder and constructs a [`ListChannelMessagesInput`](crate::operation::list_channel_messages::ListChannelMessagesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_channel_messages::ListChannelMessagesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_channel_messages::ListChannelMessagesInput {
                channel_arn: self.channel_arn,
                sort_order: self.sort_order,
                not_before: self.not_before,
                not_after: self.not_after,
                max_results: self.max_results,
                next_token: self.next_token,
                chime_bearer: self.chime_bearer,
            },
        )
    }
}
impl ::std::fmt::Debug for ListChannelMessagesInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ListChannelMessagesInputBuilder");
        formatter.field("channel_arn", &self.channel_arn);
        formatter.field("sort_order", &self.sort_order);
        formatter.field("not_before", &self.not_before);
        formatter.field("not_after", &self.not_after);
        formatter.field("max_results", &self.max_results);
        formatter.field("next_token", &"*** Sensitive Data Redacted ***");
        formatter.field("chime_bearer", &self.chime_bearer);
        formatter.finish()
    }
}
