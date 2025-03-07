// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the request to resend the confirmation code.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ResendConfirmationCodeInput {
    /// <p>The ID of the client associated with the user pool.</p>
    #[doc(hidden)]
    pub client_id: ::std::option::Option<::std::string::String>,
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    #[doc(hidden)]
    pub secret_hash: ::std::option::Option<::std::string::String>,
    /// <p>Contextual data about your user session, such as the device fingerprint, IP address, or location. Amazon Cognito advanced security evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p>
    #[doc(hidden)]
    pub user_context_data: ::std::option::Option<crate::types::UserContextDataType>,
    /// <p>The <code>username</code> attribute of the user to whom you want to resend a confirmation code.</p>
    #[doc(hidden)]
    pub username: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Pinpoint analytics metadata that contributes to your metrics for <code>ResendConfirmationCode</code> calls.</p>
    #[doc(hidden)]
    pub analytics_metadata: ::std::option::Option<crate::types::AnalyticsMetadataType>,
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the ResendConfirmationCode API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your ResendConfirmationCode request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    #[doc(hidden)]
    pub client_metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl ResendConfirmationCodeInput {
    /// <p>The ID of the client associated with the user pool.</p>
    pub fn client_id(&self) -> ::std::option::Option<&str> {
        self.client_id.as_deref()
    }
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    pub fn secret_hash(&self) -> ::std::option::Option<&str> {
        self.secret_hash.as_deref()
    }
    /// <p>Contextual data about your user session, such as the device fingerprint, IP address, or location. Amazon Cognito advanced security evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p>
    pub fn user_context_data(&self) -> ::std::option::Option<&crate::types::UserContextDataType> {
        self.user_context_data.as_ref()
    }
    /// <p>The <code>username</code> attribute of the user to whom you want to resend a confirmation code.</p>
    pub fn username(&self) -> ::std::option::Option<&str> {
        self.username.as_deref()
    }
    /// <p>The Amazon Pinpoint analytics metadata that contributes to your metrics for <code>ResendConfirmationCode</code> calls.</p>
    pub fn analytics_metadata(
        &self,
    ) -> ::std::option::Option<&crate::types::AnalyticsMetadataType> {
        self.analytics_metadata.as_ref()
    }
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the ResendConfirmationCode API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your ResendConfirmationCode request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    pub fn client_metadata(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.client_metadata.as_ref()
    }
}
impl ::std::fmt::Debug for ResendConfirmationCodeInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ResendConfirmationCodeInput");
        formatter.field("client_id", &"*** Sensitive Data Redacted ***");
        formatter.field("secret_hash", &"*** Sensitive Data Redacted ***");
        formatter.field("user_context_data", &self.user_context_data);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.field("analytics_metadata", &self.analytics_metadata);
        formatter.field("client_metadata", &self.client_metadata);
        formatter.finish()
    }
}
impl ResendConfirmationCodeInput {
    /// Creates a new builder-style object to manufacture [`ResendConfirmationCodeInput`](crate::operation::resend_confirmation_code::ResendConfirmationCodeInput).
    pub fn builder(
    ) -> crate::operation::resend_confirmation_code::builders::ResendConfirmationCodeInputBuilder
    {
        crate::operation::resend_confirmation_code::builders::ResendConfirmationCodeInputBuilder::default()
    }
}

/// A builder for [`ResendConfirmationCodeInput`](crate::operation::resend_confirmation_code::ResendConfirmationCodeInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ResendConfirmationCodeInputBuilder {
    pub(crate) client_id: ::std::option::Option<::std::string::String>,
    pub(crate) secret_hash: ::std::option::Option<::std::string::String>,
    pub(crate) user_context_data: ::std::option::Option<crate::types::UserContextDataType>,
    pub(crate) username: ::std::option::Option<::std::string::String>,
    pub(crate) analytics_metadata: ::std::option::Option<crate::types::AnalyticsMetadataType>,
    pub(crate) client_metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl ResendConfirmationCodeInputBuilder {
    /// <p>The ID of the client associated with the user pool.</p>
    pub fn client_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the client associated with the user pool.</p>
    pub fn set_client_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_id = input;
        self
    }
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    pub fn secret_hash(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.secret_hash = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A keyed-hash message authentication code (HMAC) calculated using the secret key of a user pool client and username plus the client ID in the message.</p>
    pub fn set_secret_hash(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.secret_hash = input;
        self
    }
    /// <p>Contextual data about your user session, such as the device fingerprint, IP address, or location. Amazon Cognito advanced security evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p>
    pub fn user_context_data(mut self, input: crate::types::UserContextDataType) -> Self {
        self.user_context_data = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contextual data about your user session, such as the device fingerprint, IP address, or location. Amazon Cognito advanced security evaluates the risk of an authentication event based on the context that your app generates and passes to Amazon Cognito when it makes API requests.</p>
    pub fn set_user_context_data(
        mut self,
        input: ::std::option::Option<crate::types::UserContextDataType>,
    ) -> Self {
        self.user_context_data = input;
        self
    }
    /// <p>The <code>username</code> attribute of the user to whom you want to resend a confirmation code.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.username = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <code>username</code> attribute of the user to whom you want to resend a confirmation code.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.username = input;
        self
    }
    /// <p>The Amazon Pinpoint analytics metadata that contributes to your metrics for <code>ResendConfirmationCode</code> calls.</p>
    pub fn analytics_metadata(mut self, input: crate::types::AnalyticsMetadataType) -> Self {
        self.analytics_metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Amazon Pinpoint analytics metadata that contributes to your metrics for <code>ResendConfirmationCode</code> calls.</p>
    pub fn set_analytics_metadata(
        mut self,
        input: ::std::option::Option<crate::types::AnalyticsMetadataType>,
    ) -> Self {
        self.analytics_metadata = input;
        self
    }
    /// Adds a key-value pair to `client_metadata`.
    ///
    /// To override the contents of this collection use [`set_client_metadata`](Self::set_client_metadata).
    ///
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the ResendConfirmationCode API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your ResendConfirmationCode request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    pub fn client_metadata(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.client_metadata.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.client_metadata = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the ResendConfirmationCode API action, Amazon Cognito invokes the function that is assigned to the <i>custom message</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your ResendConfirmationCode request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-identity-pools-working-with-aws-lambda-triggers.html"> Customizing user pool Workflows with Lambda Triggers</a> in the <i>Amazon Cognito Developer Guide</i>.</p> <note>
    /// <p>When you use the ClientMetadata parameter, remember that Amazon Cognito won't do the following:</p>
    /// <ul>
    /// <li> <p>Store the ClientMetadata value. This data is available only to Lambda triggers that are assigned to a user pool to support custom workflows. If your user pool configuration doesn't include triggers, the ClientMetadata parameter serves no purpose.</p> </li>
    /// <li> <p>Validate the ClientMetadata value.</p> </li>
    /// <li> <p>Encrypt the ClientMetadata value. Don't use Amazon Cognito to provide sensitive information.</p> </li>
    /// </ul>
    /// </note>
    pub fn set_client_metadata(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.client_metadata = input;
        self
    }
    /// Consumes the builder and constructs a [`ResendConfirmationCodeInput`](crate::operation::resend_confirmation_code::ResendConfirmationCodeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::resend_confirmation_code::ResendConfirmationCodeInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::resend_confirmation_code::ResendConfirmationCodeInput {
                client_id: self.client_id,
                secret_hash: self.secret_hash,
                user_context_data: self.user_context_data,
                username: self.username,
                analytics_metadata: self.analytics_metadata,
                client_metadata: self.client_metadata,
            },
        )
    }
}
impl ::std::fmt::Debug for ResendConfirmationCodeInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ResendConfirmationCodeInputBuilder");
        formatter.field("client_id", &"*** Sensitive Data Redacted ***");
        formatter.field("secret_hash", &"*** Sensitive Data Redacted ***");
        formatter.field("user_context_data", &self.user_context_data);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.field("analytics_metadata", &self.analytics_metadata);
        formatter.field("client_metadata", &self.client_metadata);
        formatter.finish()
    }
}
