// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Lambda function used to transform objects through an Object Lambda Access Point.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsLambdaTransformation {
    /// <p>The Amazon Resource Name (ARN) of the Lambda function.</p>
    #[doc(hidden)]
    pub function_arn: ::std::option::Option<::std::string::String>,
    /// <p>Additional JSON that provides supplemental data to the Lambda function used to transform objects.</p>
    #[doc(hidden)]
    pub function_payload: ::std::option::Option<::std::string::String>,
}
impl AwsLambdaTransformation {
    /// <p>The Amazon Resource Name (ARN) of the Lambda function.</p>
    pub fn function_arn(&self) -> ::std::option::Option<&str> {
        self.function_arn.as_deref()
    }
    /// <p>Additional JSON that provides supplemental data to the Lambda function used to transform objects.</p>
    pub fn function_payload(&self) -> ::std::option::Option<&str> {
        self.function_payload.as_deref()
    }
}
impl AwsLambdaTransformation {
    /// Creates a new builder-style object to manufacture [`AwsLambdaTransformation`](crate::types::AwsLambdaTransformation).
    pub fn builder() -> crate::types::builders::AwsLambdaTransformationBuilder {
        crate::types::builders::AwsLambdaTransformationBuilder::default()
    }
}

/// A builder for [`AwsLambdaTransformation`](crate::types::AwsLambdaTransformation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsLambdaTransformationBuilder {
    pub(crate) function_arn: ::std::option::Option<::std::string::String>,
    pub(crate) function_payload: ::std::option::Option<::std::string::String>,
}
impl AwsLambdaTransformationBuilder {
    /// <p>The Amazon Resource Name (ARN) of the Lambda function.</p>
    pub fn function_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.function_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function.</p>
    pub fn set_function_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.function_arn = input;
        self
    }
    /// <p>Additional JSON that provides supplemental data to the Lambda function used to transform objects.</p>
    pub fn function_payload(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.function_payload = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Additional JSON that provides supplemental data to the Lambda function used to transform objects.</p>
    pub fn set_function_payload(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.function_payload = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsLambdaTransformation`](crate::types::AwsLambdaTransformation).
    pub fn build(self) -> crate::types::AwsLambdaTransformation {
        crate::types::AwsLambdaTransformation {
            function_arn: self.function_arn,
            function_payload: self.function_payload,
        }
    }
}
