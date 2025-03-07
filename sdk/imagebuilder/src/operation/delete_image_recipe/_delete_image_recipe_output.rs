// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteImageRecipeOutput {
    /// <p>The request ID that uniquely identifies this request.</p>
    #[doc(hidden)]
    pub request_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the image recipe that was deleted.</p>
    #[doc(hidden)]
    pub image_recipe_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteImageRecipeOutput {
    /// <p>The request ID that uniquely identifies this request.</p>
    pub fn request_id(&self) -> ::std::option::Option<&str> {
        self.request_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the image recipe that was deleted.</p>
    pub fn image_recipe_arn(&self) -> ::std::option::Option<&str> {
        self.image_recipe_arn.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DeleteImageRecipeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteImageRecipeOutput {
    /// Creates a new builder-style object to manufacture [`DeleteImageRecipeOutput`](crate::operation::delete_image_recipe::DeleteImageRecipeOutput).
    pub fn builder(
    ) -> crate::operation::delete_image_recipe::builders::DeleteImageRecipeOutputBuilder {
        crate::operation::delete_image_recipe::builders::DeleteImageRecipeOutputBuilder::default()
    }
}

/// A builder for [`DeleteImageRecipeOutput`](crate::operation::delete_image_recipe::DeleteImageRecipeOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DeleteImageRecipeOutputBuilder {
    pub(crate) request_id: ::std::option::Option<::std::string::String>,
    pub(crate) image_recipe_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteImageRecipeOutputBuilder {
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
    /// <p>The Amazon Resource Name (ARN) of the image recipe that was deleted.</p>
    pub fn image_recipe_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.image_recipe_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the image recipe that was deleted.</p>
    pub fn set_image_recipe_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.image_recipe_arn = input;
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
    /// Consumes the builder and constructs a [`DeleteImageRecipeOutput`](crate::operation::delete_image_recipe::DeleteImageRecipeOutput).
    pub fn build(self) -> crate::operation::delete_image_recipe::DeleteImageRecipeOutput {
        crate::operation::delete_image_recipe::DeleteImageRecipeOutput {
            request_id: self.request_id,
            image_recipe_arn: self.image_recipe_arn,
            _request_id: self._request_id,
        }
    }
}
