// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the request to create a user in the specified user pool.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct AdminCreateUserInput {
    /// <p>The user pool ID for the user pool where the user will be created.</p>
    #[doc(hidden)]
    pub user_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username can't be changed.</p>
    #[doc(hidden)]
    pub username: ::std::option::Option<::std::string::String>,
    /// <p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p>
    /// <p>In your call to <code>AdminCreateUser</code>, you can set the <code>email_verified</code> attribute to <code>True</code>, and you can set the <code>phone_number_verified</code> attribute to <code>True</code>. You can also do this by calling <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminUpdateUserAttributes.html">AdminUpdateUserAttributes</a>.</p>
    /// <ul>
    /// <li> <p> <b>email</b>: The email address of the user to whom the message that contains the code and username will be sent. Required if the <code>email_verified</code> attribute is set to <code>True</code>, or if <code>"EMAIL"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li>
    /// <li> <p> <b>phone_number</b>: The phone number of the user to whom the message that contains the code and username will be sent. Required if the <code>phone_number_verified</code> attribute is set to <code>True</code>, or if <code>"SMS"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub user_attributes: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>,
    /// <p>The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. For example, you might choose to allow or disallow user sign-up based on the user's domain.</p>
    /// <p>To configure custom validation, you must create a Pre Sign-up Lambda trigger for the user pool as described in the Amazon Cognito Developer Guide. The Lambda trigger receives the validation data and uses it in the validation process.</p>
    /// <p>The user's validation data isn't persisted.</p>
    #[doc(hidden)]
    pub validation_data: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>,
    /// <p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p>
    /// <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page, along with a new password to be used in all future sign-ins.</p>
    /// <p>This parameter isn't required. If you don't specify a value, Amazon Cognito generates one for you.</p>
    /// <p>The temporary password can only be used until the user account expiration limit that you specified when you created the user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>"RESEND"</code> for the <code>MessageAction</code> parameter.</p>
    #[doc(hidden)]
    pub temporary_password: ::std::option::Option<::std::string::String>,
    /// <p>This parameter is used only if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p>
    /// <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the UserAttributes parameter already exists as an alias with a different user, the API call will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias.</p>
    /// <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>
    #[doc(hidden)]
    pub force_alias_creation: bool,
    /// <p>Set to <code>RESEND</code> to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to <code>SUPPRESS</code> to suppress sending the message. You can specify only one value.</p>
    #[doc(hidden)]
    pub message_action: ::std::option::Option<crate::types::MessageActionType>,
    /// <p>Specify <code>"EMAIL"</code> if email will be used to send the welcome message. Specify <code>"SMS"</code> if the phone number will be used. The default value is <code>"SMS"</code>. You can specify more than one value.</p>
    #[doc(hidden)]
    pub desired_delivery_mediums:
        ::std::option::Option<::std::vec::Vec<crate::types::DeliveryMediumType>>,
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminCreateUser API action, Amazon Cognito invokes the function that is assigned to the <i>pre sign-up</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminCreateUser request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
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
impl AdminCreateUserInput {
    /// <p>The user pool ID for the user pool where the user will be created.</p>
    pub fn user_pool_id(&self) -> ::std::option::Option<&str> {
        self.user_pool_id.as_deref()
    }
    /// <p>The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username can't be changed.</p>
    pub fn username(&self) -> ::std::option::Option<&str> {
        self.username.as_deref()
    }
    /// <p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p>
    /// <p>In your call to <code>AdminCreateUser</code>, you can set the <code>email_verified</code> attribute to <code>True</code>, and you can set the <code>phone_number_verified</code> attribute to <code>True</code>. You can also do this by calling <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminUpdateUserAttributes.html">AdminUpdateUserAttributes</a>.</p>
    /// <ul>
    /// <li> <p> <b>email</b>: The email address of the user to whom the message that contains the code and username will be sent. Required if the <code>email_verified</code> attribute is set to <code>True</code>, or if <code>"EMAIL"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li>
    /// <li> <p> <b>phone_number</b>: The phone number of the user to whom the message that contains the code and username will be sent. Required if the <code>phone_number_verified</code> attribute is set to <code>True</code>, or if <code>"SMS"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li>
    /// </ul>
    pub fn user_attributes(&self) -> ::std::option::Option<&[crate::types::AttributeType]> {
        self.user_attributes.as_deref()
    }
    /// <p>The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. For example, you might choose to allow or disallow user sign-up based on the user's domain.</p>
    /// <p>To configure custom validation, you must create a Pre Sign-up Lambda trigger for the user pool as described in the Amazon Cognito Developer Guide. The Lambda trigger receives the validation data and uses it in the validation process.</p>
    /// <p>The user's validation data isn't persisted.</p>
    pub fn validation_data(&self) -> ::std::option::Option<&[crate::types::AttributeType]> {
        self.validation_data.as_deref()
    }
    /// <p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p>
    /// <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page, along with a new password to be used in all future sign-ins.</p>
    /// <p>This parameter isn't required. If you don't specify a value, Amazon Cognito generates one for you.</p>
    /// <p>The temporary password can only be used until the user account expiration limit that you specified when you created the user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>"RESEND"</code> for the <code>MessageAction</code> parameter.</p>
    pub fn temporary_password(&self) -> ::std::option::Option<&str> {
        self.temporary_password.as_deref()
    }
    /// <p>This parameter is used only if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p>
    /// <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the UserAttributes parameter already exists as an alias with a different user, the API call will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias.</p>
    /// <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>
    pub fn force_alias_creation(&self) -> bool {
        self.force_alias_creation
    }
    /// <p>Set to <code>RESEND</code> to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to <code>SUPPRESS</code> to suppress sending the message. You can specify only one value.</p>
    pub fn message_action(&self) -> ::std::option::Option<&crate::types::MessageActionType> {
        self.message_action.as_ref()
    }
    /// <p>Specify <code>"EMAIL"</code> if email will be used to send the welcome message. Specify <code>"SMS"</code> if the phone number will be used. The default value is <code>"SMS"</code>. You can specify more than one value.</p>
    pub fn desired_delivery_mediums(
        &self,
    ) -> ::std::option::Option<&[crate::types::DeliveryMediumType]> {
        self.desired_delivery_mediums.as_deref()
    }
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminCreateUser API action, Amazon Cognito invokes the function that is assigned to the <i>pre sign-up</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminCreateUser request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
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
impl ::std::fmt::Debug for AdminCreateUserInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdminCreateUserInput");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.field("user_attributes", &self.user_attributes);
        formatter.field("validation_data", &self.validation_data);
        formatter.field("temporary_password", &"*** Sensitive Data Redacted ***");
        formatter.field("force_alias_creation", &self.force_alias_creation);
        formatter.field("message_action", &self.message_action);
        formatter.field("desired_delivery_mediums", &self.desired_delivery_mediums);
        formatter.field("client_metadata", &self.client_metadata);
        formatter.finish()
    }
}
impl AdminCreateUserInput {
    /// Creates a new builder-style object to manufacture [`AdminCreateUserInput`](crate::operation::admin_create_user::AdminCreateUserInput).
    pub fn builder() -> crate::operation::admin_create_user::builders::AdminCreateUserInputBuilder {
        crate::operation::admin_create_user::builders::AdminCreateUserInputBuilder::default()
    }
}

/// A builder for [`AdminCreateUserInput`](crate::operation::admin_create_user::AdminCreateUserInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct AdminCreateUserInputBuilder {
    pub(crate) user_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) username: ::std::option::Option<::std::string::String>,
    pub(crate) user_attributes: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>,
    pub(crate) validation_data: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>,
    pub(crate) temporary_password: ::std::option::Option<::std::string::String>,
    pub(crate) force_alias_creation: ::std::option::Option<bool>,
    pub(crate) message_action: ::std::option::Option<crate::types::MessageActionType>,
    pub(crate) desired_delivery_mediums:
        ::std::option::Option<::std::vec::Vec<crate::types::DeliveryMediumType>>,
    pub(crate) client_metadata: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl AdminCreateUserInputBuilder {
    /// <p>The user pool ID for the user pool where the user will be created.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user pool ID for the user pool where the user will be created.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_pool_id = input;
        self
    }
    /// <p>The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username can't be changed.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.username = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The username for the user. Must be unique within the user pool. Must be a UTF-8 string between 1 and 128 characters. After the user is created, the username can't be changed.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.username = input;
        self
    }
    /// Appends an item to `user_attributes`.
    ///
    /// To override the contents of this collection use [`set_user_attributes`](Self::set_user_attributes).
    ///
    /// <p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p>
    /// <p>In your call to <code>AdminCreateUser</code>, you can set the <code>email_verified</code> attribute to <code>True</code>, and you can set the <code>phone_number_verified</code> attribute to <code>True</code>. You can also do this by calling <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminUpdateUserAttributes.html">AdminUpdateUserAttributes</a>.</p>
    /// <ul>
    /// <li> <p> <b>email</b>: The email address of the user to whom the message that contains the code and username will be sent. Required if the <code>email_verified</code> attribute is set to <code>True</code>, or if <code>"EMAIL"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li>
    /// <li> <p> <b>phone_number</b>: The phone number of the user to whom the message that contains the code and username will be sent. Required if the <code>phone_number_verified</code> attribute is set to <code>True</code>, or if <code>"SMS"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li>
    /// </ul>
    pub fn user_attributes(mut self, input: crate::types::AttributeType) -> Self {
        let mut v = self.user_attributes.unwrap_or_default();
        v.push(input);
        self.user_attributes = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of name-value pairs that contain user attributes and attribute values to be set for the user to be created. You can create a user without specifying any attributes other than <code>Username</code>. However, any attributes that you specify as required (when creating a user pool or in the <b>Attributes</b> tab of the console) either you should supply (in your call to <code>AdminCreateUser</code>) or the user should supply (when they sign up in response to your welcome message).</p>
    /// <p>For custom attributes, you must prepend the <code>custom:</code> prefix to the attribute name.</p>
    /// <p>To send a message inviting the user to sign up, you must specify the user's email address or phone number. You can do this in your call to AdminCreateUser or in the <b>Users</b> tab of the Amazon Cognito console for managing your user pools.</p>
    /// <p>In your call to <code>AdminCreateUser</code>, you can set the <code>email_verified</code> attribute to <code>True</code>, and you can set the <code>phone_number_verified</code> attribute to <code>True</code>. You can also do this by calling <a href="https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminUpdateUserAttributes.html">AdminUpdateUserAttributes</a>.</p>
    /// <ul>
    /// <li> <p> <b>email</b>: The email address of the user to whom the message that contains the code and username will be sent. Required if the <code>email_verified</code> attribute is set to <code>True</code>, or if <code>"EMAIL"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li>
    /// <li> <p> <b>phone_number</b>: The phone number of the user to whom the message that contains the code and username will be sent. Required if the <code>phone_number_verified</code> attribute is set to <code>True</code>, or if <code>"SMS"</code> is specified in the <code>DesiredDeliveryMediums</code> parameter.</p> </li>
    /// </ul>
    pub fn set_user_attributes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>,
    ) -> Self {
        self.user_attributes = input;
        self
    }
    /// Appends an item to `validation_data`.
    ///
    /// To override the contents of this collection use [`set_validation_data`](Self::set_validation_data).
    ///
    /// <p>The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. For example, you might choose to allow or disallow user sign-up based on the user's domain.</p>
    /// <p>To configure custom validation, you must create a Pre Sign-up Lambda trigger for the user pool as described in the Amazon Cognito Developer Guide. The Lambda trigger receives the validation data and uses it in the validation process.</p>
    /// <p>The user's validation data isn't persisted.</p>
    pub fn validation_data(mut self, input: crate::types::AttributeType) -> Self {
        let mut v = self.validation_data.unwrap_or_default();
        v.push(input);
        self.validation_data = ::std::option::Option::Some(v);
        self
    }
    /// <p>The user's validation data. This is an array of name-value pairs that contain user attributes and attribute values that you can use for custom validation, such as restricting the types of user accounts that can be registered. For example, you might choose to allow or disallow user sign-up based on the user's domain.</p>
    /// <p>To configure custom validation, you must create a Pre Sign-up Lambda trigger for the user pool as described in the Amazon Cognito Developer Guide. The Lambda trigger receives the validation data and uses it in the validation process.</p>
    /// <p>The user's validation data isn't persisted.</p>
    pub fn set_validation_data(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AttributeType>>,
    ) -> Self {
        self.validation_data = input;
        self
    }
    /// <p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p>
    /// <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page, along with a new password to be used in all future sign-ins.</p>
    /// <p>This parameter isn't required. If you don't specify a value, Amazon Cognito generates one for you.</p>
    /// <p>The temporary password can only be used until the user account expiration limit that you specified when you created the user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>"RESEND"</code> for the <code>MessageAction</code> parameter.</p>
    pub fn temporary_password(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.temporary_password = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user's temporary password. This password must conform to the password policy that you specified when you created the user pool.</p>
    /// <p>The temporary password is valid only once. To complete the Admin Create User flow, the user must enter the temporary password in the sign-in page, along with a new password to be used in all future sign-ins.</p>
    /// <p>This parameter isn't required. If you don't specify a value, Amazon Cognito generates one for you.</p>
    /// <p>The temporary password can only be used until the user account expiration limit that you specified when you created the user pool. To reset the account after that time limit, you must call <code>AdminCreateUser</code> again, specifying <code>"RESEND"</code> for the <code>MessageAction</code> parameter.</p>
    pub fn set_temporary_password(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.temporary_password = input;
        self
    }
    /// <p>This parameter is used only if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p>
    /// <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the UserAttributes parameter already exists as an alias with a different user, the API call will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias.</p>
    /// <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>
    pub fn force_alias_creation(mut self, input: bool) -> Self {
        self.force_alias_creation = ::std::option::Option::Some(input);
        self
    }
    /// <p>This parameter is used only if the <code>phone_number_verified</code> or <code>email_verified</code> attribute is set to <code>True</code>. Otherwise, it is ignored.</p>
    /// <p>If this parameter is set to <code>True</code> and the phone number or email address specified in the UserAttributes parameter already exists as an alias with a different user, the API call will migrate the alias from the previous user to the newly created user. The previous user will no longer be able to log in using that alias.</p>
    /// <p>If this parameter is set to <code>False</code>, the API throws an <code>AliasExistsException</code> error if the alias already exists. The default value is <code>False</code>.</p>
    pub fn set_force_alias_creation(mut self, input: ::std::option::Option<bool>) -> Self {
        self.force_alias_creation = input;
        self
    }
    /// <p>Set to <code>RESEND</code> to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to <code>SUPPRESS</code> to suppress sending the message. You can specify only one value.</p>
    pub fn message_action(mut self, input: crate::types::MessageActionType) -> Self {
        self.message_action = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set to <code>RESEND</code> to resend the invitation message to a user that already exists and reset the expiration limit on the user's account. Set to <code>SUPPRESS</code> to suppress sending the message. You can specify only one value.</p>
    pub fn set_message_action(
        mut self,
        input: ::std::option::Option<crate::types::MessageActionType>,
    ) -> Self {
        self.message_action = input;
        self
    }
    /// Appends an item to `desired_delivery_mediums`.
    ///
    /// To override the contents of this collection use [`set_desired_delivery_mediums`](Self::set_desired_delivery_mediums).
    ///
    /// <p>Specify <code>"EMAIL"</code> if email will be used to send the welcome message. Specify <code>"SMS"</code> if the phone number will be used. The default value is <code>"SMS"</code>. You can specify more than one value.</p>
    pub fn desired_delivery_mediums(mut self, input: crate::types::DeliveryMediumType) -> Self {
        let mut v = self.desired_delivery_mediums.unwrap_or_default();
        v.push(input);
        self.desired_delivery_mediums = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specify <code>"EMAIL"</code> if email will be used to send the welcome message. Specify <code>"SMS"</code> if the phone number will be used. The default value is <code>"SMS"</code>. You can specify more than one value.</p>
    pub fn set_desired_delivery_mediums(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::DeliveryMediumType>>,
    ) -> Self {
        self.desired_delivery_mediums = input;
        self
    }
    /// Adds a key-value pair to `client_metadata`.
    ///
    /// To override the contents of this collection use [`set_client_metadata`](Self::set_client_metadata).
    ///
    /// <p>A map of custom key-value pairs that you can provide as input for any custom workflows that this action triggers.</p>
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminCreateUser API action, Amazon Cognito invokes the function that is assigned to the <i>pre sign-up</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminCreateUser request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
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
    /// <p>You create custom workflows by assigning Lambda functions to user pool triggers. When you use the AdminCreateUser API action, Amazon Cognito invokes the function that is assigned to the <i>pre sign-up</i> trigger. When Amazon Cognito invokes this function, it passes a JSON payload, which the function receives as input. This payload contains a <code>clientMetadata</code> attribute, which provides the data that you assigned to the ClientMetadata parameter in your AdminCreateUser request. In your function code in Lambda, you can process the <code>clientMetadata</code> value to enhance your workflow for your specific needs.</p>
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
    /// Consumes the builder and constructs a [`AdminCreateUserInput`](crate::operation::admin_create_user::AdminCreateUserInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::admin_create_user::AdminCreateUserInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::admin_create_user::AdminCreateUserInput {
            user_pool_id: self.user_pool_id,
            username: self.username,
            user_attributes: self.user_attributes,
            validation_data: self.validation_data,
            temporary_password: self.temporary_password,
            force_alias_creation: self.force_alias_creation.unwrap_or_default(),
            message_action: self.message_action,
            desired_delivery_mediums: self.desired_delivery_mediums,
            client_metadata: self.client_metadata,
        })
    }
}
impl ::std::fmt::Debug for AdminCreateUserInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("AdminCreateUserInputBuilder");
        formatter.field("user_pool_id", &self.user_pool_id);
        formatter.field("username", &"*** Sensitive Data Redacted ***");
        formatter.field("user_attributes", &self.user_attributes);
        formatter.field("validation_data", &self.validation_data);
        formatter.field("temporary_password", &"*** Sensitive Data Redacted ***");
        formatter.field("force_alias_creation", &self.force_alias_creation);
        formatter.field("message_action", &self.message_action);
        formatter.field("desired_delivery_mediums", &self.desired_delivery_mediums);
        formatter.field("client_metadata", &self.client_metadata);
        formatter.finish()
    }
}
