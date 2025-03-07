// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The AccountLimit data type.</p>
/// <p>CloudFormation has the following limits per account:</p>
/// <ul>
/// <li> <p>Number of concurrent resources</p> </li>
/// <li> <p>Number of stacks</p> </li>
/// <li> <p>Number of stack outputs</p> </li>
/// </ul>
/// <p>For more information about these account limits, and other CloudFormation limits, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/cloudformation-limits.html">CloudFormation quotas</a> in the <i>CloudFormation User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AccountLimit {
    /// <p>The name of the account limit.</p>
    /// <p>Values: <code>ConcurrentResourcesLimit</code> | <code>StackLimit</code> | <code>StackOutputsLimit</code> </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The value that's associated with the account limit name.</p>
    #[doc(hidden)]
    pub value: ::std::option::Option<i32>,
}
impl AccountLimit {
    /// <p>The name of the account limit.</p>
    /// <p>Values: <code>ConcurrentResourcesLimit</code> | <code>StackLimit</code> | <code>StackOutputsLimit</code> </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The value that's associated with the account limit name.</p>
    pub fn value(&self) -> ::std::option::Option<i32> {
        self.value
    }
}
impl AccountLimit {
    /// Creates a new builder-style object to manufacture [`AccountLimit`](crate::types::AccountLimit).
    pub fn builder() -> crate::types::builders::AccountLimitBuilder {
        crate::types::builders::AccountLimitBuilder::default()
    }
}

/// A builder for [`AccountLimit`](crate::types::AccountLimit).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AccountLimitBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<i32>,
}
impl AccountLimitBuilder {
    /// <p>The name of the account limit.</p>
    /// <p>Values: <code>ConcurrentResourcesLimit</code> | <code>StackLimit</code> | <code>StackOutputsLimit</code> </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the account limit.</p>
    /// <p>Values: <code>ConcurrentResourcesLimit</code> | <code>StackLimit</code> | <code>StackOutputsLimit</code> </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The value that's associated with the account limit name.</p>
    pub fn value(mut self, input: i32) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The value that's associated with the account limit name.</p>
    pub fn set_value(mut self, input: ::std::option::Option<i32>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`AccountLimit`](crate::types::AccountLimit).
    pub fn build(self) -> crate::types::AccountLimit {
        crate::types::AccountLimit {
            name: self.name,
            value: self.value,
        }
    }
}
