// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The specification of an output context that is set when an intent is fulfilled.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OutputContext {
    /// <p>The name of the context.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The number of seconds that the context should be active after it is first sent in a <code>PostContent</code> or <code>PostText</code> response. You can set the value between 5 and 86,400 seconds (24 hours).</p>
    #[doc(hidden)]
    pub time_to_live_in_seconds: ::std::option::Option<i32>,
    /// <p>The number of conversation turns that the context should be active. A conversation turn is one <code>PostContent</code> or <code>PostText</code> request and the corresponding response from Amazon Lex.</p>
    #[doc(hidden)]
    pub turns_to_live: ::std::option::Option<i32>,
}
impl OutputContext {
    /// <p>The name of the context.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The number of seconds that the context should be active after it is first sent in a <code>PostContent</code> or <code>PostText</code> response. You can set the value between 5 and 86,400 seconds (24 hours).</p>
    pub fn time_to_live_in_seconds(&self) -> ::std::option::Option<i32> {
        self.time_to_live_in_seconds
    }
    /// <p>The number of conversation turns that the context should be active. A conversation turn is one <code>PostContent</code> or <code>PostText</code> request and the corresponding response from Amazon Lex.</p>
    pub fn turns_to_live(&self) -> ::std::option::Option<i32> {
        self.turns_to_live
    }
}
impl OutputContext {
    /// Creates a new builder-style object to manufacture [`OutputContext`](crate::types::OutputContext).
    pub fn builder() -> crate::types::builders::OutputContextBuilder {
        crate::types::builders::OutputContextBuilder::default()
    }
}

/// A builder for [`OutputContext`](crate::types::OutputContext).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OutputContextBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) time_to_live_in_seconds: ::std::option::Option<i32>,
    pub(crate) turns_to_live: ::std::option::Option<i32>,
}
impl OutputContextBuilder {
    /// <p>The name of the context.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the context.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The number of seconds that the context should be active after it is first sent in a <code>PostContent</code> or <code>PostText</code> response. You can set the value between 5 and 86,400 seconds (24 hours).</p>
    pub fn time_to_live_in_seconds(mut self, input: i32) -> Self {
        self.time_to_live_in_seconds = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of seconds that the context should be active after it is first sent in a <code>PostContent</code> or <code>PostText</code> response. You can set the value between 5 and 86,400 seconds (24 hours).</p>
    pub fn set_time_to_live_in_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.time_to_live_in_seconds = input;
        self
    }
    /// <p>The number of conversation turns that the context should be active. A conversation turn is one <code>PostContent</code> or <code>PostText</code> request and the corresponding response from Amazon Lex.</p>
    pub fn turns_to_live(mut self, input: i32) -> Self {
        self.turns_to_live = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of conversation turns that the context should be active. A conversation turn is one <code>PostContent</code> or <code>PostText</code> request and the corresponding response from Amazon Lex.</p>
    pub fn set_turns_to_live(mut self, input: ::std::option::Option<i32>) -> Self {
        self.turns_to_live = input;
        self
    }
    /// Consumes the builder and constructs a [`OutputContext`](crate::types::OutputContext).
    pub fn build(self) -> crate::types::OutputContext {
        crate::types::OutputContext {
            name: self.name,
            time_to_live_in_seconds: self.time_to_live_in_seconds,
            turns_to_live: self.turns_to_live,
        }
    }
}
