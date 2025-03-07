// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateBillingGroupInput {
    /// <p> The token that is needed to support idempotency. Idempotency isn't currently supported, but will be implemented in a future update. </p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p> The billing group name. The names must be unique. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p> The set of accounts that will be under the billing group. The set of accounts resemble the linked accounts in a consolidated family. </p>
    #[doc(hidden)]
    pub account_grouping: ::std::option::Option<crate::types::AccountGrouping>,
    /// <p> The preferences and settings that will be used to compute the Amazon Web Services charges for a billing group. </p>
    #[doc(hidden)]
    pub computation_preference: ::std::option::Option<crate::types::ComputationPreference>,
    /// <p> The account ID that serves as the main account in a billing group. </p>
    #[doc(hidden)]
    pub primary_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The description of the billing group. </p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p> A map that contains tag keys and tag values that are attached to a billing group. This feature isn't available during the beta. </p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateBillingGroupInput {
    /// <p> The token that is needed to support idempotency. Idempotency isn't currently supported, but will be implemented in a future update. </p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p> The billing group name. The names must be unique. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p> The set of accounts that will be under the billing group. The set of accounts resemble the linked accounts in a consolidated family. </p>
    pub fn account_grouping(&self) -> ::std::option::Option<&crate::types::AccountGrouping> {
        self.account_grouping.as_ref()
    }
    /// <p> The preferences and settings that will be used to compute the Amazon Web Services charges for a billing group. </p>
    pub fn computation_preference(
        &self,
    ) -> ::std::option::Option<&crate::types::ComputationPreference> {
        self.computation_preference.as_ref()
    }
    /// <p> The account ID that serves as the main account in a billing group. </p>
    pub fn primary_account_id(&self) -> ::std::option::Option<&str> {
        self.primary_account_id.as_deref()
    }
    /// <p>The description of the billing group. </p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p> A map that contains tag keys and tag values that are attached to a billing group. This feature isn't available during the beta. </p>
    pub fn tags(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.tags.as_ref()
    }
}
impl ::std::fmt::Debug for CreateBillingGroupInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateBillingGroupInput");
        formatter.field("client_token", &self.client_token);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("account_grouping", &self.account_grouping);
        formatter.field("computation_preference", &self.computation_preference);
        formatter.field("primary_account_id", &self.primary_account_id);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
impl CreateBillingGroupInput {
    /// Creates a new builder-style object to manufacture [`CreateBillingGroupInput`](crate::operation::create_billing_group::CreateBillingGroupInput).
    pub fn builder(
    ) -> crate::operation::create_billing_group::builders::CreateBillingGroupInputBuilder {
        crate::operation::create_billing_group::builders::CreateBillingGroupInputBuilder::default()
    }
}

/// A builder for [`CreateBillingGroupInput`](crate::operation::create_billing_group::CreateBillingGroupInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CreateBillingGroupInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) account_grouping: ::std::option::Option<crate::types::AccountGrouping>,
    pub(crate) computation_preference: ::std::option::Option<crate::types::ComputationPreference>,
    pub(crate) primary_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl CreateBillingGroupInputBuilder {
    /// <p> The token that is needed to support idempotency. Idempotency isn't currently supported, but will be implemented in a future update. </p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The token that is needed to support idempotency. Idempotency isn't currently supported, but will be implemented in a future update. </p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p> The billing group name. The names must be unique. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The billing group name. The names must be unique. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p> The set of accounts that will be under the billing group. The set of accounts resemble the linked accounts in a consolidated family. </p>
    pub fn account_grouping(mut self, input: crate::types::AccountGrouping) -> Self {
        self.account_grouping = ::std::option::Option::Some(input);
        self
    }
    /// <p> The set of accounts that will be under the billing group. The set of accounts resemble the linked accounts in a consolidated family. </p>
    pub fn set_account_grouping(
        mut self,
        input: ::std::option::Option<crate::types::AccountGrouping>,
    ) -> Self {
        self.account_grouping = input;
        self
    }
    /// <p> The preferences and settings that will be used to compute the Amazon Web Services charges for a billing group. </p>
    pub fn computation_preference(mut self, input: crate::types::ComputationPreference) -> Self {
        self.computation_preference = ::std::option::Option::Some(input);
        self
    }
    /// <p> The preferences and settings that will be used to compute the Amazon Web Services charges for a billing group. </p>
    pub fn set_computation_preference(
        mut self,
        input: ::std::option::Option<crate::types::ComputationPreference>,
    ) -> Self {
        self.computation_preference = input;
        self
    }
    /// <p> The account ID that serves as the main account in a billing group. </p>
    pub fn primary_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.primary_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The account ID that serves as the main account in a billing group. </p>
    pub fn set_primary_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.primary_account_id = input;
        self
    }
    /// <p>The description of the billing group. </p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the billing group. </p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p> A map that contains tag keys and tag values that are attached to a billing group. This feature isn't available during the beta. </p>
    pub fn tags(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p> A map that contains tag keys and tag values that are attached to a billing group. This feature isn't available during the beta. </p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateBillingGroupInput`](crate::operation::create_billing_group::CreateBillingGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_billing_group::CreateBillingGroupInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_billing_group::CreateBillingGroupInput {
                client_token: self.client_token,
                name: self.name,
                account_grouping: self.account_grouping,
                computation_preference: self.computation_preference,
                primary_account_id: self.primary_account_id,
                description: self.description,
                tags: self.tags,
            },
        )
    }
}
impl ::std::fmt::Debug for CreateBillingGroupInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateBillingGroupInputBuilder");
        formatter.field("client_token", &self.client_token);
        formatter.field("name", &"*** Sensitive Data Redacted ***");
        formatter.field("account_grouping", &self.account_grouping);
        formatter.field("computation_preference", &self.computation_preference);
        formatter.field("primary_account_id", &self.primary_account_id);
        formatter.field("description", &"*** Sensitive Data Redacted ***");
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
