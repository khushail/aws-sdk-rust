// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchDisableStandardsInput {
    /// <p>The ARNs of the standards subscriptions to disable.</p>
    #[doc(hidden)]
    pub standards_subscription_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchDisableStandardsInput {
    /// <p>The ARNs of the standards subscriptions to disable.</p>
    pub fn standards_subscription_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.standards_subscription_arns.as_deref()
    }
}
impl BatchDisableStandardsInput {
    /// Creates a new builder-style object to manufacture [`BatchDisableStandardsInput`](crate::operation::batch_disable_standards::BatchDisableStandardsInput).
    pub fn builder(
    ) -> crate::operation::batch_disable_standards::builders::BatchDisableStandardsInputBuilder
    {
        crate::operation::batch_disable_standards::builders::BatchDisableStandardsInputBuilder::default()
    }
}

/// A builder for [`BatchDisableStandardsInput`](crate::operation::batch_disable_standards::BatchDisableStandardsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchDisableStandardsInputBuilder {
    pub(crate) standards_subscription_arns:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl BatchDisableStandardsInputBuilder {
    /// Appends an item to `standards_subscription_arns`.
    ///
    /// To override the contents of this collection use [`set_standards_subscription_arns`](Self::set_standards_subscription_arns).
    ///
    /// <p>The ARNs of the standards subscriptions to disable.</p>
    pub fn standards_subscription_arns(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.standards_subscription_arns.unwrap_or_default();
        v.push(input.into());
        self.standards_subscription_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The ARNs of the standards subscriptions to disable.</p>
    pub fn set_standards_subscription_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.standards_subscription_arns = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchDisableStandardsInput`](crate::operation::batch_disable_standards::BatchDisableStandardsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_disable_standards::BatchDisableStandardsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::batch_disable_standards::BatchDisableStandardsInput {
                standards_subscription_arns: self.standards_subscription_arns,
            },
        )
    }
}
