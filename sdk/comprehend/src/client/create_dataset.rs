// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateDataset`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`flywheel_arn(impl ::std::convert::Into<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::flywheel_arn) / [`set_flywheel_arn(Option<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_flywheel_arn): <p>The Amazon Resource Number (ARN) of the flywheel of the flywheel to receive the data.</p>
    ///   - [`dataset_name(impl ::std::convert::Into<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::dataset_name) / [`set_dataset_name(Option<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_dataset_name): <p>Name of the dataset.</p>
    ///   - [`dataset_type(DatasetType)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::dataset_type) / [`set_dataset_type(Option<DatasetType>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_dataset_type): <p>The dataset type. You can specify that the data in a dataset is for training the model or for testing the model.</p>
    ///   - [`description(impl ::std::convert::Into<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_description): <p>Description of the dataset.</p>
    ///   - [`input_data_config(DatasetInputDataConfig)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::input_data_config) / [`set_input_data_config(Option<DatasetInputDataConfig>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_input_data_config): <p>Information about the input data configuration. The type of input data varies based on the format of the input and whether the data is for a classifier model or an entity recognition model.</p>
    ///   - [`client_request_token(impl ::std::convert::Into<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::client_request_token) / [`set_client_request_token(Option<String>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_client_request_token): <p>A unique identifier for the request. If you don't set the client request token, Amazon Comprehend generates one.</p>
    ///   - [`tags(Vec<Tag>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::tags) / [`set_tags(Option<Vec<Tag>>)`](crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::set_tags): <p>Tags for the dataset.</p>
    /// - On success, responds with [`CreateDatasetOutput`](crate::operation::create_dataset::CreateDatasetOutput) with field(s):
    ///   - [`dataset_arn(Option<String>)`](crate::operation::create_dataset::CreateDatasetOutput::dataset_arn): <p>The ARN of the dataset.</p>
    /// - On failure, responds with [`SdkError<CreateDatasetError>`](crate::operation::create_dataset::CreateDatasetError)
    pub fn create_dataset(
        &self,
    ) -> crate::operation::create_dataset::builders::CreateDatasetFluentBuilder {
        crate::operation::create_dataset::builders::CreateDatasetFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
