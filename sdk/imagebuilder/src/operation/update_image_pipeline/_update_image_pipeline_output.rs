// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateImagePipelineOutput {
    /// <p>The request ID that uniquely identifies this request.</p>
    #[doc(hidden)]
    pub request_id: ::std::option::Option<::std::string::String>,
    /// <p>The idempotency token used to make this request idempotent.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that was updated by this request.</p>
    #[doc(hidden)]
    pub image_pipeline_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateImagePipelineOutput {
    /// <p>The request ID that uniquely identifies this request.</p>
    pub fn request_id(&self) -> ::std::option::Option<&str> {
        self.request_id.as_deref()
    }
    /// <p>The idempotency token used to make this request idempotent.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that was updated by this request.</p>
    pub fn image_pipeline_arn(&self) -> ::std::option::Option<&str> {
        self.image_pipeline_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for UpdateImagePipelineOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateImagePipelineOutput {
    /// Creates a new builder-style object to manufacture [`UpdateImagePipelineOutput`](crate::operation::update_image_pipeline::UpdateImagePipelineOutput).
    pub fn builder(
    ) -> crate::operation::update_image_pipeline::builders::UpdateImagePipelineOutputBuilder {
        crate::operation::update_image_pipeline::builders::UpdateImagePipelineOutputBuilder::default(
        )
    }
}

/// A builder for [`UpdateImagePipelineOutput`](crate::operation::update_image_pipeline::UpdateImagePipelineOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateImagePipelineOutputBuilder {
    pub(crate) request_id: ::std::option::Option<::std::string::String>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) image_pipeline_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl UpdateImagePipelineOutputBuilder {
    /// <p>The request ID that uniquely identifies this request.</p>
    pub fn request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.request_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The request ID that uniquely identifies this request.</p>
    pub fn set_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.request_id = input;
        self
    }
    /// <p>The idempotency token used to make this request idempotent.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The idempotency token used to make this request idempotent.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that was updated by this request.</p>
    pub fn image_pipeline_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.image_pipeline_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the image pipeline that was updated by this request.</p>
    pub fn set_image_pipeline_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.image_pipeline_arn = input;
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
    /// Consumes the builder and constructs a [`UpdateImagePipelineOutput`](crate::operation::update_image_pipeline::UpdateImagePipelineOutput).
    pub fn build(self) -> crate::operation::update_image_pipeline::UpdateImagePipelineOutput {
        crate::operation::update_image_pipeline::UpdateImagePipelineOutput {
            request_id: self.request_id,
            client_token: self.client_token,
            image_pipeline_arn: self.image_pipeline_arn,
            _request_id: self._request_id,
        }
    }
}
