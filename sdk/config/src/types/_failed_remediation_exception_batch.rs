// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>List of each of the failed remediation exceptions with specific reasons.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FailedRemediationExceptionBatch {
    /// <p>Returns a failure message. For example, the auto-remediation has failed.</p>
    #[doc(hidden)]
    pub failure_message: ::std::option::Option<::std::string::String>,
    /// <p>Returns remediation exception resource key object of the failed items.</p>
    #[doc(hidden)]
    pub failed_items: ::std::option::Option<::std::vec::Vec<crate::types::RemediationException>>,
}
impl FailedRemediationExceptionBatch {
    /// <p>Returns a failure message. For example, the auto-remediation has failed.</p>
    pub fn failure_message(&self) -> ::std::option::Option<&str> {
        self.failure_message.as_deref()
    }
    /// <p>Returns remediation exception resource key object of the failed items.</p>
    pub fn failed_items(&self) -> ::std::option::Option<&[crate::types::RemediationException]> {
        self.failed_items.as_deref()
    }
}
impl FailedRemediationExceptionBatch {
    /// Creates a new builder-style object to manufacture [`FailedRemediationExceptionBatch`](crate::types::FailedRemediationExceptionBatch).
    pub fn builder() -> crate::types::builders::FailedRemediationExceptionBatchBuilder {
        crate::types::builders::FailedRemediationExceptionBatchBuilder::default()
    }
}

/// A builder for [`FailedRemediationExceptionBatch`](crate::types::FailedRemediationExceptionBatch).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FailedRemediationExceptionBatchBuilder {
    pub(crate) failure_message: ::std::option::Option<::std::string::String>,
    pub(crate) failed_items:
        ::std::option::Option<::std::vec::Vec<crate::types::RemediationException>>,
}
impl FailedRemediationExceptionBatchBuilder {
    /// <p>Returns a failure message. For example, the auto-remediation has failed.</p>
    pub fn failure_message(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.failure_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Returns a failure message. For example, the auto-remediation has failed.</p>
    pub fn set_failure_message(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.failure_message = input;
        self
    }
    /// Appends an item to `failed_items`.
    ///
    /// To override the contents of this collection use [`set_failed_items`](Self::set_failed_items).
    ///
    /// <p>Returns remediation exception resource key object of the failed items.</p>
    pub fn failed_items(mut self, input: crate::types::RemediationException) -> Self {
        let mut v = self.failed_items.unwrap_or_default();
        v.push(input);
        self.failed_items = ::std::option::Option::Some(v);
        self
    }
    /// <p>Returns remediation exception resource key object of the failed items.</p>
    pub fn set_failed_items(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RemediationException>>,
    ) -> Self {
        self.failed_items = input;
        self
    }
    /// Consumes the builder and constructs a [`FailedRemediationExceptionBatch`](crate::types::FailedRemediationExceptionBatch).
    pub fn build(self) -> crate::types::FailedRemediationExceptionBatch {
        crate::types::FailedRemediationExceptionBatch {
            failure_message: self.failure_message,
            failed_items: self.failed_items,
        }
    }
}
