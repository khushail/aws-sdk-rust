// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides the details of the <code>LambdaFunctionStarted</code> event. It isn't set for other event types.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LambdaFunctionStartedEventAttributes {
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    #[doc(hidden)]
    pub scheduled_event_id: i64,
}
impl LambdaFunctionStartedEventAttributes {
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    pub fn scheduled_event_id(&self) -> i64 {
        self.scheduled_event_id
    }
}
impl LambdaFunctionStartedEventAttributes {
    /// Creates a new builder-style object to manufacture [`LambdaFunctionStartedEventAttributes`](crate::types::LambdaFunctionStartedEventAttributes).
    pub fn builder() -> crate::types::builders::LambdaFunctionStartedEventAttributesBuilder {
        crate::types::builders::LambdaFunctionStartedEventAttributesBuilder::default()
    }
}

/// A builder for [`LambdaFunctionStartedEventAttributes`](crate::types::LambdaFunctionStartedEventAttributes).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct LambdaFunctionStartedEventAttributesBuilder {
    pub(crate) scheduled_event_id: ::std::option::Option<i64>,
}
impl LambdaFunctionStartedEventAttributesBuilder {
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    pub fn scheduled_event_id(mut self, input: i64) -> Self {
        self.scheduled_event_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The ID of the <code>LambdaFunctionScheduled</code> event that was recorded when this activity task was scheduled. To help diagnose issues, use this information to trace back the chain of events leading up to this event.</p>
    pub fn set_scheduled_event_id(mut self, input: ::std::option::Option<i64>) -> Self {
        self.scheduled_event_id = input;
        self
    }
    /// Consumes the builder and constructs a [`LambdaFunctionStartedEventAttributes`](crate::types::LambdaFunctionStartedEventAttributes).
    pub fn build(self) -> crate::types::LambdaFunctionStartedEventAttributes {
        crate::types::LambdaFunctionStartedEventAttributes {
            scheduled_event_id: self.scheduled_event_id.unwrap_or_default(),
        }
    }
}
