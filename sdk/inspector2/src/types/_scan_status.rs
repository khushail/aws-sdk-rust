// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The status of the scan.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScanStatus {
    /// <p>The status code of the scan.</p>
    #[doc(hidden)]
    pub status_code: ::std::option::Option<crate::types::ScanStatusCode>,
    /// <p>The reason for the scan.</p>
    #[doc(hidden)]
    pub reason: ::std::option::Option<crate::types::ScanStatusReason>,
}
impl ScanStatus {
    /// <p>The status code of the scan.</p>
    pub fn status_code(&self) -> ::std::option::Option<&crate::types::ScanStatusCode> {
        self.status_code.as_ref()
    }
    /// <p>The reason for the scan.</p>
    pub fn reason(&self) -> ::std::option::Option<&crate::types::ScanStatusReason> {
        self.reason.as_ref()
    }
}
impl ScanStatus {
    /// Creates a new builder-style object to manufacture [`ScanStatus`](crate::types::ScanStatus).
    pub fn builder() -> crate::types::builders::ScanStatusBuilder {
        crate::types::builders::ScanStatusBuilder::default()
    }
}

/// A builder for [`ScanStatus`](crate::types::ScanStatus).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ScanStatusBuilder {
    pub(crate) status_code: ::std::option::Option<crate::types::ScanStatusCode>,
    pub(crate) reason: ::std::option::Option<crate::types::ScanStatusReason>,
}
impl ScanStatusBuilder {
    /// <p>The status code of the scan.</p>
    pub fn status_code(mut self, input: crate::types::ScanStatusCode) -> Self {
        self.status_code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status code of the scan.</p>
    pub fn set_status_code(
        mut self,
        input: ::std::option::Option<crate::types::ScanStatusCode>,
    ) -> Self {
        self.status_code = input;
        self
    }
    /// <p>The reason for the scan.</p>
    pub fn reason(mut self, input: crate::types::ScanStatusReason) -> Self {
        self.reason = ::std::option::Option::Some(input);
        self
    }
    /// <p>The reason for the scan.</p>
    pub fn set_reason(
        mut self,
        input: ::std::option::Option<crate::types::ScanStatusReason>,
    ) -> Self {
        self.reason = input;
        self
    }
    /// Consumes the builder and constructs a [`ScanStatus`](crate::types::ScanStatus).
    pub fn build(self) -> crate::types::ScanStatus {
        crate::types::ScanStatus {
            status_code: self.status_code,
            reason: self.reason,
        }
    }
}
