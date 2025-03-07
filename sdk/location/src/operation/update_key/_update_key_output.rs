// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateKeyOutput {
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li> <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code> </p> </li>
    /// </ul>
    #[doc(hidden)]
    pub key_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the API key resource.</p>
    #[doc(hidden)]
    pub key_name: ::std::option::Option<::std::string::String>,
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    #[doc(hidden)]
    pub update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl UpdateKeyOutput {
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li> <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code> </p> </li>
    /// </ul>
    pub fn key_arn(&self) -> ::std::option::Option<&str> {
        self.key_arn.as_deref()
    }
    /// <p>The name of the API key resource.</p>
    pub fn key_name(&self) -> ::std::option::Option<&str> {
        self.key_name.as_deref()
    }
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn update_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.update_time.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateKeyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateKeyOutput {
    /// Creates a new builder-style object to manufacture [`UpdateKeyOutput`](crate::operation::update_key::UpdateKeyOutput).
    pub fn builder() -> crate::operation::update_key::builders::UpdateKeyOutputBuilder {
        crate::operation::update_key::builders::UpdateKeyOutputBuilder::default()
    }
}

/// A builder for [`UpdateKeyOutput`](crate::operation::update_key::UpdateKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateKeyOutputBuilder {
    pub(crate) key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) key_name: ::std::option::Option<::std::string::String>,
    pub(crate) update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl UpdateKeyOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li> <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code> </p> </li>
    /// </ul>
    pub fn key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li> <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code> </p> </li>
    /// </ul>
    pub fn set_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_arn = input;
        self
    }
    /// <p>The name of the API key resource.</p>
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the API key resource.</p>
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_name = input;
        self
    }
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn set_update_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.update_time = input;
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
    /// Consumes the builder and constructs a [`UpdateKeyOutput`](crate::operation::update_key::UpdateKeyOutput).
    pub fn build(self) -> crate::operation::update_key::UpdateKeyOutput {
        crate::operation::update_key::UpdateKeyOutput {
            key_arn: self.key_arn,
            key_name: self.key_name,
            update_time: self.update_time,
            _request_id: self._request_id,
        }
    }
}
