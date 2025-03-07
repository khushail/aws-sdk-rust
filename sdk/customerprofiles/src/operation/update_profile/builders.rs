// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_profile::_update_profile_output::UpdateProfileOutputBuilder;

pub use crate::operation::update_profile::_update_profile_input::UpdateProfileInputBuilder;

/// Fluent builder constructing a request to `UpdateProfile`.
///
/// <p>Updates the properties of a profile. The ProfileId is required for updating a customer profile.</p>
/// <p>When calling the UpdateProfile API, specifying an empty string value means that any existing value will be removed. Not specifying a string value means that any value already there will be kept.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_profile::builders::UpdateProfileInputBuilder,
}
impl UpdateProfileFluentBuilder {
    /// Creates a new `UpdateProfile`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_profile::UpdateProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_profile::UpdateProfileError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_profile::UpdateProfileOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_profile::UpdateProfileError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_profile::UpdateProfileOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::update_profile::UpdateProfileError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::update_profile::UpdateProfile,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::update_profile::UpdateProfileError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The unique name of the domain.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The unique name of the domain.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The unique identifier of a customer profile.</p>
    pub fn profile_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.profile_id(input.into());
        self
    }
    /// <p>The unique identifier of a customer profile.</p>
    pub fn set_profile_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_profile_id(input);
        self
    }
    /// <p>Any additional information relevant to the customer’s profile.</p>
    pub fn additional_information(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.additional_information(input.into());
        self
    }
    /// <p>Any additional information relevant to the customer’s profile.</p>
    pub fn set_additional_information(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_additional_information(input);
        self
    }
    /// <p>A unique account number that you have given to the customer.</p>
    pub fn account_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.account_number(input.into());
        self
    }
    /// <p>A unique account number that you have given to the customer.</p>
    pub fn set_account_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_account_number(input);
        self
    }
    /// <p>The type of profile used to describe the customer.</p>
    pub fn party_type(mut self, input: crate::types::PartyType) -> Self {
        self.inner = self.inner.party_type(input);
        self
    }
    /// <p>The type of profile used to describe the customer.</p>
    pub fn set_party_type(mut self, input: ::std::option::Option<crate::types::PartyType>) -> Self {
        self.inner = self.inner.set_party_type(input);
        self
    }
    /// <p>The name of the customer’s business.</p>
    pub fn business_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.business_name(input.into());
        self
    }
    /// <p>The name of the customer’s business.</p>
    pub fn set_business_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_business_name(input);
        self
    }
    /// <p>The customer’s first name.</p>
    pub fn first_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.first_name(input.into());
        self
    }
    /// <p>The customer’s first name.</p>
    pub fn set_first_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_first_name(input);
        self
    }
    /// <p>The customer’s middle name.</p>
    pub fn middle_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.middle_name(input.into());
        self
    }
    /// <p>The customer’s middle name.</p>
    pub fn set_middle_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_middle_name(input);
        self
    }
    /// <p>The customer’s last name.</p>
    pub fn last_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.last_name(input.into());
        self
    }
    /// <p>The customer’s last name.</p>
    pub fn set_last_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_last_name(input);
        self
    }
    /// <p>The customer’s birth date. </p>
    pub fn birth_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.birth_date(input.into());
        self
    }
    /// <p>The customer’s birth date. </p>
    pub fn set_birth_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_birth_date(input);
        self
    }
    /// <p>The gender with which the customer identifies. </p>
    pub fn gender(mut self, input: crate::types::Gender) -> Self {
        self.inner = self.inner.gender(input);
        self
    }
    /// <p>The gender with which the customer identifies. </p>
    pub fn set_gender(mut self, input: ::std::option::Option<crate::types::Gender>) -> Self {
        self.inner = self.inner.set_gender(input);
        self
    }
    /// <p>The customer’s phone number, which has not been specified as a mobile, home, or business number. </p>
    pub fn phone_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.phone_number(input.into());
        self
    }
    /// <p>The customer’s phone number, which has not been specified as a mobile, home, or business number. </p>
    pub fn set_phone_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_phone_number(input);
        self
    }
    /// <p>The customer’s mobile phone number.</p>
    pub fn mobile_phone_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.mobile_phone_number(input.into());
        self
    }
    /// <p>The customer’s mobile phone number.</p>
    pub fn set_mobile_phone_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_mobile_phone_number(input);
        self
    }
    /// <p>The customer’s home phone number.</p>
    pub fn home_phone_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.home_phone_number(input.into());
        self
    }
    /// <p>The customer’s home phone number.</p>
    pub fn set_home_phone_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_home_phone_number(input);
        self
    }
    /// <p>The customer’s business phone number.</p>
    pub fn business_phone_number(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.business_phone_number(input.into());
        self
    }
    /// <p>The customer’s business phone number.</p>
    pub fn set_business_phone_number(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_business_phone_number(input);
        self
    }
    /// <p>The customer’s email address, which has not been specified as a personal or business address. </p>
    pub fn email_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.email_address(input.into());
        self
    }
    /// <p>The customer’s email address, which has not been specified as a personal or business address. </p>
    pub fn set_email_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_email_address(input);
        self
    }
    /// <p>The customer’s personal email address.</p>
    pub fn personal_email_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.personal_email_address(input.into());
        self
    }
    /// <p>The customer’s personal email address.</p>
    pub fn set_personal_email_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_personal_email_address(input);
        self
    }
    /// <p>The customer’s business email address.</p>
    pub fn business_email_address(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.business_email_address(input.into());
        self
    }
    /// <p>The customer’s business email address.</p>
    pub fn set_business_email_address(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_business_email_address(input);
        self
    }
    /// <p>A generic address associated with the customer that is not mailing, shipping, or billing.</p>
    pub fn address(mut self, input: crate::types::UpdateAddress) -> Self {
        self.inner = self.inner.address(input);
        self
    }
    /// <p>A generic address associated with the customer that is not mailing, shipping, or billing.</p>
    pub fn set_address(
        mut self,
        input: ::std::option::Option<crate::types::UpdateAddress>,
    ) -> Self {
        self.inner = self.inner.set_address(input);
        self
    }
    /// <p>The customer’s shipping address.</p>
    pub fn shipping_address(mut self, input: crate::types::UpdateAddress) -> Self {
        self.inner = self.inner.shipping_address(input);
        self
    }
    /// <p>The customer’s shipping address.</p>
    pub fn set_shipping_address(
        mut self,
        input: ::std::option::Option<crate::types::UpdateAddress>,
    ) -> Self {
        self.inner = self.inner.set_shipping_address(input);
        self
    }
    /// <p>The customer’s mailing address.</p>
    pub fn mailing_address(mut self, input: crate::types::UpdateAddress) -> Self {
        self.inner = self.inner.mailing_address(input);
        self
    }
    /// <p>The customer’s mailing address.</p>
    pub fn set_mailing_address(
        mut self,
        input: ::std::option::Option<crate::types::UpdateAddress>,
    ) -> Self {
        self.inner = self.inner.set_mailing_address(input);
        self
    }
    /// <p>The customer’s billing address.</p>
    pub fn billing_address(mut self, input: crate::types::UpdateAddress) -> Self {
        self.inner = self.inner.billing_address(input);
        self
    }
    /// <p>The customer’s billing address.</p>
    pub fn set_billing_address(
        mut self,
        input: ::std::option::Option<crate::types::UpdateAddress>,
    ) -> Self {
        self.inner = self.inner.set_billing_address(input);
        self
    }
    /// Adds a key-value pair to `Attributes`.
    ///
    /// To override the contents of this collection use [`set_attributes`](Self::set_attributes).
    ///
    /// <p>A key value pair of attributes of a customer profile.</p>
    pub fn attributes(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.attributes(k.into(), v.into());
        self
    }
    /// <p>A key value pair of attributes of a customer profile.</p>
    pub fn set_attributes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_attributes(input);
        self
    }
    /// <p>An alternative to <code>PartyType</code> which accepts any string as input.</p>
    pub fn party_type_string(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.party_type_string(input.into());
        self
    }
    /// <p>An alternative to <code>PartyType</code> which accepts any string as input.</p>
    pub fn set_party_type_string(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_party_type_string(input);
        self
    }
    /// <p>An alternative to <code>Gender</code> which accepts any string as input.</p>
    pub fn gender_string(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.gender_string(input.into());
        self
    }
    /// <p>An alternative to <code>Gender</code> which accepts any string as input.</p>
    pub fn set_gender_string(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_gender_string(input);
        self
    }
}
