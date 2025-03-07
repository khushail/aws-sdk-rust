// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateDatasetInput {
    /// <p>The Amazon Resource Number (ARN) of the flywheel of the flywheel to receive the data.</p>
    #[doc(hidden)]
    pub flywheel_arn: ::std::option::Option<::std::string::String>,
    /// <p>Name of the dataset.</p>
    #[doc(hidden)]
    pub dataset_name: ::std::option::Option<::std::string::String>,
    /// <p>The dataset type. You can specify that the data in a dataset is for training the model or for testing the model.</p>
    #[doc(hidden)]
    pub dataset_type: ::std::option::Option<crate::types::DatasetType>,
    /// <p>Description of the dataset.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Information about the input data configuration. The type of input data varies based on the format of the input and whether the data is for a classifier model or an entity recognition model.</p>
    #[doc(hidden)]
    pub input_data_config: ::std::option::Option<crate::types::DatasetInputDataConfig>,
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    #[doc(hidden)]
    pub client_request_token: ::std::option::Option<::std::string::String>,
    /// <p>Tags for the dataset.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateDatasetInput {
    /// <p>The Amazon Resource Number (ARN) of the flywheel of the flywheel to receive the data.</p>
    pub fn flywheel_arn(&self) -> ::std::option::Option<&str> {
        self.flywheel_arn.as_deref()
    }
    /// <p>Name of the dataset.</p>
    pub fn dataset_name(&self) -> ::std::option::Option<&str> {
        self.dataset_name.as_deref()
    }
    /// <p>The dataset type. You can specify that the data in a dataset is for training the model or for testing the model.</p>
    pub fn dataset_type(&self) -> ::std::option::Option<&crate::types::DatasetType> {
        self.dataset_type.as_ref()
    }
    /// <p>Description of the dataset.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Information about the input data configuration. The type of input data varies based on the format of the input and whether the data is for a classifier model or an entity recognition model.</p>
    pub fn input_data_config(
        &self,
    ) -> ::std::option::Option<&crate::types::DatasetInputDataConfig> {
        self.input_data_config.as_ref()
    }
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
    /// <p>Tags for the dataset.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl CreateDatasetInput {
    /// Creates a new builder-style object to manufacture [`CreateDatasetInput`](crate::operation::create_dataset::CreateDatasetInput).
    pub fn builder() -> crate::operation::create_dataset::builders::CreateDatasetInputBuilder {
        crate::operation::create_dataset::builders::CreateDatasetInputBuilder::default()
    }
}

/// A builder for [`CreateDatasetInput`](crate::operation::create_dataset::CreateDatasetInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateDatasetInputBuilder {
    pub(crate) flywheel_arn: ::std::option::Option<::std::string::String>,
    pub(crate) dataset_name: ::std::option::Option<::std::string::String>,
    pub(crate) dataset_type: ::std::option::Option<crate::types::DatasetType>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) input_data_config: ::std::option::Option<crate::types::DatasetInputDataConfig>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateDatasetInputBuilder {
    /// <p>The Amazon Resource Number (ARN) of the flywheel of the flywheel to receive the data.</p>
    pub fn flywheel_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.flywheel_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Number (ARN) of the flywheel of the flywheel to receive the data.</p>
    pub fn set_flywheel_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.flywheel_arn = input;
        self
    }
    /// <p>Name of the dataset.</p>
    pub fn dataset_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dataset_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name of the dataset.</p>
    pub fn set_dataset_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dataset_name = input;
        self
    }
    /// <p>The dataset type. You can specify that the data in a dataset is for training the model or for testing the model.</p>
    pub fn dataset_type(mut self, input: crate::types::DatasetType) -> Self {
        self.dataset_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The dataset type. You can specify that the data in a dataset is for training the model or for testing the model.</p>
    pub fn set_dataset_type(
        mut self,
        input: ::std::option::Option<crate::types::DatasetType>,
    ) -> Self {
        self.dataset_type = input;
        self
    }
    /// <p>Description of the dataset.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Description of the dataset.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Information about the input data configuration. The type of input data varies based on the format of the input and whether the data is for a classifier model or an entity recognition model.</p>
    pub fn input_data_config(mut self, input: crate::types::DatasetInputDataConfig) -> Self {
        self.input_data_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the input data configuration. The type of input data varies based on the format of the input and whether the data is for a classifier model or an entity recognition model.</p>
    pub fn set_input_data_config(
        mut self,
        input: ::std::option::Option<crate::types::DatasetInputDataConfig>,
    ) -> Self {
        self.input_data_config = input;
        self
    }
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    pub fn client_request_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    pub fn set_client_request_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.client_request_token = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags for the dataset.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Tags for the dataset.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateDatasetInput`](crate::operation::create_dataset::CreateDatasetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_dataset::CreateDatasetInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_dataset::CreateDatasetInput {
            flywheel_arn: self.flywheel_arn,
            dataset_name: self.dataset_name,
            dataset_type: self.dataset_type,
            description: self.description,
            input_data_config: self.input_data_config,
            client_request_token: self.client_request_token,
            tags: self.tags,
        })
    }
}
