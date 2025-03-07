// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configures inspection of the response body. WAF can inspect the first 65,536 bytes (64 KB) of the response body. This is part of the <code>ResponseInspection</code> configuration for <code>AWSManagedRulesATPRuleSet</code>. </p> <note>
/// <p>Response inspection is available only in web ACLs that protect Amazon CloudFront distributions.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResponseInspectionBodyContains {
    /// <p>Strings in the body of the response that indicate a successful login attempt. To be counted as a successful login, the string can be anywhere in the body and must be an exact match, including case. Each string must be unique among the success and failure strings. </p>
    /// <p>JSON example: <code>"SuccessStrings": [ "Login successful", "Welcome to our site!" ]</code> </p>
    #[doc(hidden)]
    pub success_strings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Strings in the body of the response that indicate a failed login attempt. To be counted as a failed login, the string can be anywhere in the body and must be an exact match, including case. Each string must be unique among the success and failure strings. </p>
    /// <p>JSON example: <code>"FailureStrings": [ "Login failed" ]</code> </p>
    #[doc(hidden)]
    pub failure_strings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ResponseInspectionBodyContains {
    /// <p>Strings in the body of the response that indicate a successful login attempt. To be counted as a successful login, the string can be anywhere in the body and must be an exact match, including case. Each string must be unique among the success and failure strings. </p>
    /// <p>JSON example: <code>"SuccessStrings": [ "Login successful", "Welcome to our site!" ]</code> </p>
    pub fn success_strings(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.success_strings.as_deref()
    }
    /// <p>Strings in the body of the response that indicate a failed login attempt. To be counted as a failed login, the string can be anywhere in the body and must be an exact match, including case. Each string must be unique among the success and failure strings. </p>
    /// <p>JSON example: <code>"FailureStrings": [ "Login failed" ]</code> </p>
    pub fn failure_strings(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.failure_strings.as_deref()
    }
}
impl ResponseInspectionBodyContains {
    /// Creates a new builder-style object to manufacture [`ResponseInspectionBodyContains`](crate::types::ResponseInspectionBodyContains).
    pub fn builder() -> crate::types::builders::ResponseInspectionBodyContainsBuilder {
        crate::types::builders::ResponseInspectionBodyContainsBuilder::default()
    }
}

/// A builder for [`ResponseInspectionBodyContains`](crate::types::ResponseInspectionBodyContains).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ResponseInspectionBodyContainsBuilder {
    pub(crate) success_strings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) failure_strings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl ResponseInspectionBodyContainsBuilder {
    /// Appends an item to `success_strings`.
    ///
    /// To override the contents of this collection use [`set_success_strings`](Self::set_success_strings).
    ///
    /// <p>Strings in the body of the response that indicate a successful login attempt. To be counted as a successful login, the string can be anywhere in the body and must be an exact match, including case. Each string must be unique among the success and failure strings. </p>
    /// <p>JSON example: <code>"SuccessStrings": [ "Login successful", "Welcome to our site!" ]</code> </p>
    pub fn success_strings(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.success_strings.unwrap_or_default();
        v.push(input.into());
        self.success_strings = ::std::option::Option::Some(v);
        self
    }
    /// <p>Strings in the body of the response that indicate a successful login attempt. To be counted as a successful login, the string can be anywhere in the body and must be an exact match, including case. Each string must be unique among the success and failure strings. </p>
    /// <p>JSON example: <code>"SuccessStrings": [ "Login successful", "Welcome to our site!" ]</code> </p>
    pub fn set_success_strings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.success_strings = input;
        self
    }
    /// Appends an item to `failure_strings`.
    ///
    /// To override the contents of this collection use [`set_failure_strings`](Self::set_failure_strings).
    ///
    /// <p>Strings in the body of the response that indicate a failed login attempt. To be counted as a failed login, the string can be anywhere in the body and must be an exact match, including case. Each string must be unique among the success and failure strings. </p>
    /// <p>JSON example: <code>"FailureStrings": [ "Login failed" ]</code> </p>
    pub fn failure_strings(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.failure_strings.unwrap_or_default();
        v.push(input.into());
        self.failure_strings = ::std::option::Option::Some(v);
        self
    }
    /// <p>Strings in the body of the response that indicate a failed login attempt. To be counted as a failed login, the string can be anywhere in the body and must be an exact match, including case. Each string must be unique among the success and failure strings. </p>
    /// <p>JSON example: <code>"FailureStrings": [ "Login failed" ]</code> </p>
    pub fn set_failure_strings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.failure_strings = input;
        self
    }
    /// Consumes the builder and constructs a [`ResponseInspectionBodyContains`](crate::types::ResponseInspectionBodyContains).
    pub fn build(self) -> crate::types::ResponseInspectionBodyContains {
        crate::types::ResponseInspectionBodyContains {
            success_strings: self.success_strings,
            failure_strings: self.failure_strings,
        }
    }
}
