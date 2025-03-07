// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociatePhoneNumbersWithVoiceConnectorGroupOutput {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    #[doc(hidden)]
    pub phone_number_errors: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberError>>,
    _request_id: Option<String>,
}
impl AssociatePhoneNumbersWithVoiceConnectorGroupOutput {
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    pub fn phone_number_errors(&self) -> ::std::option::Option<&[crate::types::PhoneNumberError]> {
        self.phone_number_errors.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for AssociatePhoneNumbersWithVoiceConnectorGroupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssociatePhoneNumbersWithVoiceConnectorGroupOutput {
    /// Creates a new builder-style object to manufacture [`AssociatePhoneNumbersWithVoiceConnectorGroupOutput`](crate::operation::associate_phone_numbers_with_voice_connector_group::AssociatePhoneNumbersWithVoiceConnectorGroupOutput).
    pub fn builder() -> crate::operation::associate_phone_numbers_with_voice_connector_group::builders::AssociatePhoneNumbersWithVoiceConnectorGroupOutputBuilder{
        crate::operation::associate_phone_numbers_with_voice_connector_group::builders::AssociatePhoneNumbersWithVoiceConnectorGroupOutputBuilder::default()
    }
}

/// A builder for [`AssociatePhoneNumbersWithVoiceConnectorGroupOutput`](crate::operation::associate_phone_numbers_with_voice_connector_group::AssociatePhoneNumbersWithVoiceConnectorGroupOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AssociatePhoneNumbersWithVoiceConnectorGroupOutputBuilder {
    pub(crate) phone_number_errors:
        ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberError>>,
    _request_id: Option<String>,
}
impl AssociatePhoneNumbersWithVoiceConnectorGroupOutputBuilder {
    /// Appends an item to `phone_number_errors`.
    ///
    /// To override the contents of this collection use [`set_phone_number_errors`](Self::set_phone_number_errors).
    ///
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    pub fn phone_number_errors(mut self, input: crate::types::PhoneNumberError) -> Self {
        let mut v = self.phone_number_errors.unwrap_or_default();
        v.push(input);
        self.phone_number_errors = ::std::option::Option::Some(v);
        self
    }
    /// <p>If the action fails for one or more of the phone numbers in the request, a list of the phone numbers is returned, along with error codes and error messages.</p>
    pub fn set_phone_number_errors(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PhoneNumberError>>,
    ) -> Self {
        self.phone_number_errors = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AssociatePhoneNumbersWithVoiceConnectorGroupOutput`](crate::operation::associate_phone_numbers_with_voice_connector_group::AssociatePhoneNumbersWithVoiceConnectorGroupOutput).
    pub fn build(self) -> crate::operation::associate_phone_numbers_with_voice_connector_group::AssociatePhoneNumbersWithVoiceConnectorGroupOutput{
        crate::operation::associate_phone_numbers_with_voice_connector_group::AssociatePhoneNumbersWithVoiceConnectorGroupOutput {
            phone_number_errors: self.phone_number_errors
            ,
            _request_id: self._request_id,
        }
    }
}
