// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateDecoderManifestOutput {
    /// <p> The name of the created decoder manifest. </p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p> The ARN of the created decoder manifest. </p>
    #[doc(hidden)]
    pub arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateDecoderManifestOutput {
    /// <p> The name of the created decoder manifest. </p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p> The ARN of the created decoder manifest. </p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for CreateDecoderManifestOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateDecoderManifestOutput {
    /// Creates a new builder-style object to manufacture [`CreateDecoderManifestOutput`](crate::operation::create_decoder_manifest::CreateDecoderManifestOutput).
    pub fn builder(
    ) -> crate::operation::create_decoder_manifest::builders::CreateDecoderManifestOutputBuilder
    {
        crate::operation::create_decoder_manifest::builders::CreateDecoderManifestOutputBuilder::default()
    }
}

/// A builder for [`CreateDecoderManifestOutput`](crate::operation::create_decoder_manifest::CreateDecoderManifestOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateDecoderManifestOutputBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateDecoderManifestOutputBuilder {
    /// <p> The name of the created decoder manifest. </p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The name of the created decoder manifest. </p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p> The ARN of the created decoder manifest. </p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The ARN of the created decoder manifest. </p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
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
    /// Consumes the builder and constructs a [`CreateDecoderManifestOutput`](crate::operation::create_decoder_manifest::CreateDecoderManifestOutput).
    pub fn build(self) -> crate::operation::create_decoder_manifest::CreateDecoderManifestOutput {
        crate::operation::create_decoder_manifest::CreateDecoderManifestOutput {
            name: self.name,
            arn: self.arn,
            _request_id: self._request_id,
        }
    }
}
