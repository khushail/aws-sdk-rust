// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchEnableStandardsInput {
    /// <p>The list of standards checks to enable.</p>
    #[doc(hidden)]
    pub standards_subscription_requests:
        ::std::option::Option<::std::vec::Vec<crate::types::StandardsSubscriptionRequest>>,
}
impl BatchEnableStandardsInput {
    /// <p>The list of standards checks to enable.</p>
    pub fn standards_subscription_requests(
        &self,
    ) -> ::std::option::Option<&[crate::types::StandardsSubscriptionRequest]> {
        self.standards_subscription_requests.as_deref()
    }
}
impl BatchEnableStandardsInput {
    /// Creates a new builder-style object to manufacture [`BatchEnableStandardsInput`](crate::operation::batch_enable_standards::BatchEnableStandardsInput).
    pub fn builder(
    ) -> crate::operation::batch_enable_standards::builders::BatchEnableStandardsInputBuilder {
        crate::operation::batch_enable_standards::builders::BatchEnableStandardsInputBuilder::default()
    }
}

/// A builder for [`BatchEnableStandardsInput`](crate::operation::batch_enable_standards::BatchEnableStandardsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchEnableStandardsInputBuilder {
    pub(crate) standards_subscription_requests:
        ::std::option::Option<::std::vec::Vec<crate::types::StandardsSubscriptionRequest>>,
}
impl BatchEnableStandardsInputBuilder {
    /// Appends an item to `standards_subscription_requests`.
    ///
    /// To override the contents of this collection use [`set_standards_subscription_requests`](Self::set_standards_subscription_requests).
    ///
    /// <p>The list of standards checks to enable.</p>
    pub fn standards_subscription_requests(
        mut self,
        input: crate::types::StandardsSubscriptionRequest,
    ) -> Self {
        let mut v = self.standards_subscription_requests.unwrap_or_default();
        v.push(input);
        self.standards_subscription_requests = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of standards checks to enable.</p>
    pub fn set_standards_subscription_requests(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::StandardsSubscriptionRequest>>,
    ) -> Self {
        self.standards_subscription_requests = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchEnableStandardsInput`](crate::operation::batch_enable_standards::BatchEnableStandardsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_enable_standards::BatchEnableStandardsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::batch_enable_standards::BatchEnableStandardsInput {
                standards_subscription_requests: self.standards_subscription_requests,
            },
        )
    }
}
