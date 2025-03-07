// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DescribeRiskConfigurationInput {
    /// <p>The user pool ID.</p>
    #[doc(hidden)]
    pub user_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The app client ID.</p>
    #[doc(hidden)]
    pub client_id: ::std::option::Option<::std::string::String>,
}
impl DescribeRiskConfigurationInput {
    /// <p>The user pool ID.</p>
    pub fn user_pool_id(&self) -> ::std::option::Option<&str> {
        self.user_pool_id.as_deref()
    }
    /// <p>The app client ID.</p>
    pub fn client_id(&self) -> ::std::option::Option<&str> {
        self.client_id.as_deref()
    }
}
impl ::std::fmt::Debug for DescribeRiskConfigurationInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeRiskConfigurationInput");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("client_id", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl DescribeRiskConfigurationInput {
    /// Creates a new builder-style object to manufacture [`DescribeRiskConfigurationInput`](crate::operation::describe_risk_configuration::DescribeRiskConfigurationInput).
    pub fn builder() -> crate::operation::describe_risk_configuration::builders::DescribeRiskConfigurationInputBuilder{
        crate::operation::describe_risk_configuration::builders::DescribeRiskConfigurationInputBuilder::default()
    }
}

/// A builder for [`DescribeRiskConfigurationInput`](crate::operation::describe_risk_configuration::DescribeRiskConfigurationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct DescribeRiskConfigurationInputBuilder {
    pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) client_id: ::std::option::Option<::std::string::String>,
}
impl DescribeRiskConfigurationInputBuilder {
    /// <p>The user pool ID.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user pool ID.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_pool_id = input;
        self
    }
    /// <p>The app client ID.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The app client ID.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeRiskConfigurationInput`](crate::operation::describe_risk_configuration::DescribeRiskConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_risk_configuration::DescribeRiskConfigurationInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_risk_configuration::DescribeRiskConfigurationInput {
                user_pool_id: self.user_pool_id,
                client_id: self.client_id,
            },
        )
    }
}
impl ::std::fmt::Debug for DescribeRiskConfigurationInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeRiskConfigurationInputBuilder");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("client_id", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
