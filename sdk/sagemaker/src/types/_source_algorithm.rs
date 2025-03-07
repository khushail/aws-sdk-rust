// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies an algorithm that was used to create the model package. The algorithm must be either an algorithm resource in your SageMaker account or an algorithm in Amazon Web Services Marketplace that you are subscribed to.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SourceAlgorithm {
    /// <p>The Amazon S3 path where the model artifacts, which result from model training, are stored. This path must point to a single <code>gzip</code> compressed tar archive (<code>.tar.gz</code> suffix).</p> <note>
    /// <p>The model artifacts must be in an S3 bucket that is in the same region as the algorithm.</p>
    /// </note>
    #[doc(hidden)]
    pub model_data_url: ::std::option::Option<::std::string::String>,
    /// <p>The name of an algorithm that was used to create the model package. The algorithm must be either an algorithm resource in your SageMaker account or an algorithm in Amazon Web Services Marketplace that you are subscribed to.</p>
    #[doc(hidden)]
    pub algorithm_name: ::std::option::Option<::std::string::String>,
}
impl SourceAlgorithm {
    /// <p>The Amazon S3 path where the model artifacts, which result from model training, are stored. This path must point to a single <code>gzip</code> compressed tar archive (<code>.tar.gz</code> suffix).</p> <note>
    /// <p>The model artifacts must be in an S3 bucket that is in the same region as the algorithm.</p>
    /// </note>
    pub fn model_data_url(&self) -> ::std::option::Option<&str> {
        self.model_data_url.as_deref()
    }
    /// <p>The name of an algorithm that was used to create the model package. The algorithm must be either an algorithm resource in your SageMaker account or an algorithm in Amazon Web Services Marketplace that you are subscribed to.</p>
    pub fn algorithm_name(&self) -> ::std::option::Option<&str> {
        self.algorithm_name.as_deref()
    }
}
impl SourceAlgorithm {
    /// Creates a new builder-style object to manufacture [`SourceAlgorithm`](crate::types::SourceAlgorithm).
    pub fn builder() -> crate::types::builders::SourceAlgorithmBuilder {
        crate::types::builders::SourceAlgorithmBuilder::default()
    }
}

/// A builder for [`SourceAlgorithm`](crate::types::SourceAlgorithm).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct SourceAlgorithmBuilder {
    pub(crate) model_data_url: ::std::option::Option<::std::string::String>,
    pub(crate) algorithm_name: ::std::option::Option<::std::string::String>,
}
impl SourceAlgorithmBuilder {
    /// <p>The Amazon S3 path where the model artifacts, which result from model training, are stored. This path must point to a single <code>gzip</code> compressed tar archive (<code>.tar.gz</code> suffix).</p> <note>
    /// <p>The model artifacts must be in an S3 bucket that is in the same region as the algorithm.</p>
    /// </note>
    pub fn model_data_url(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.model_data_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon S3 path where the model artifacts, which result from model training, are stored. This path must point to a single <code>gzip</code> compressed tar archive (<code>.tar.gz</code> suffix).</p> <note>
    /// <p>The model artifacts must be in an S3 bucket that is in the same region as the algorithm.</p>
    /// </note>
    pub fn set_model_data_url(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.model_data_url = input;
        self
    }
    /// <p>The name of an algorithm that was used to create the model package. The algorithm must be either an algorithm resource in your SageMaker account or an algorithm in Amazon Web Services Marketplace that you are subscribed to.</p>
    pub fn algorithm_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.algorithm_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of an algorithm that was used to create the model package. The algorithm must be either an algorithm resource in your SageMaker account or an algorithm in Amazon Web Services Marketplace that you are subscribed to.</p>
    pub fn set_algorithm_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.algorithm_name = input;
        self
    }
    /// Consumes the builder and constructs a [`SourceAlgorithm`](crate::types::SourceAlgorithm).
    pub fn build(self) -> crate::types::SourceAlgorithm {
        crate::types::SourceAlgorithm {
            model_data_url: self.model_data_url,
            algorithm_name: self.algorithm_name,
        }
    }
}
