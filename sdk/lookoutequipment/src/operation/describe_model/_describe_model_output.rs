// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeModelOutput {
    /// <p>The name of the ML model being described. </p>
    #[doc(hidden)]
    pub model_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the ML model being described. </p>
    #[doc(hidden)]
    pub model_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the dataset being used by the ML being described. </p>
    #[doc(hidden)]
    pub dataset_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resouce Name (ARN) of the dataset used to create the ML model being described. </p>
    #[doc(hidden)]
    pub dataset_arn: ::std::option::Option<::std::string::String>,
    /// <p>A JSON description of the data that is in each time series dataset, including names, column names, and data types. </p>
    #[doc(hidden)]
    pub schema: ::std::option::Option<::std::string::String>,
    /// <p>Specifies configuration information about the labels input, including its S3 location. </p>
    #[doc(hidden)]
    pub labels_input_configuration: ::std::option::Option<crate::types::LabelsInputConfiguration>,
    /// <p> Indicates the time reference in the dataset that was used to begin the subset of training data for the ML model. </p>
    #[doc(hidden)]
    pub training_data_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> Indicates the time reference in the dataset that was used to end the subset of training data for the ML model. </p>
    #[doc(hidden)]
    pub training_data_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> Indicates the time reference in the dataset that was used to begin the subset of evaluation data for the ML model. </p>
    #[doc(hidden)]
    pub evaluation_data_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> Indicates the time reference in the dataset that was used to end the subset of evaluation data for the ML model. </p>
    #[doc(hidden)]
    pub evaluation_data_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p> The Amazon Resource Name (ARN) of a role with permission to access the data source for the ML model being described. </p>
    #[doc(hidden)]
    pub role_arn: ::std::option::Option<::std::string::String>,
    /// <p>The configuration is the <code>TargetSamplingRate</code>, which is the sampling rate of the data after post processing by Amazon Lookout for Equipment. For example, if you provide data that has been collected at a 1 second level and you want the system to resample the data at a 1 minute rate before training, the <code>TargetSamplingRate</code> is 1 minute.</p>
    /// <p>When providing a value for the <code>TargetSamplingRate</code>, you must attach the prefix "PT" to the rate you want. The value for a 1 second rate is therefore <i>PT1S</i>, the value for a 15 minute rate is <i>PT15M</i>, and the value for a 1 hour rate is <i>PT1H</i> </p>
    #[doc(hidden)]
    pub data_pre_processing_configuration:
        ::std::option::Option<crate::types::DataPreProcessingConfiguration>,
    /// <p>Specifies the current status of the model being described. Status describes the status of the most recent action of the model. </p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::ModelStatus>,
    /// <p>Indicates the time at which the training of the ML model began. </p>
    #[doc(hidden)]
    pub training_execution_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Indicates the time at which the training of the ML model was completed. </p>
    #[doc(hidden)]
    pub training_execution_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>If the training of the ML model failed, this indicates the reason for that failure. </p>
    #[doc(hidden)]
    pub failed_reason: ::std::option::Option<::std::string::String>,
    /// <p>The Model Metrics show an aggregated summary of the model's performance within the evaluation time range. This is the JSON content of the metrics created when evaluating the model. </p>
    #[doc(hidden)]
    pub model_metrics: ::std::option::Option<::std::string::String>,
    /// <p>Indicates the last time the ML model was updated. The type of update is not specified. </p>
    #[doc(hidden)]
    pub last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Indicates the time and date at which the ML model was created. </p>
    #[doc(hidden)]
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Provides the identifier of the KMS key used to encrypt model data by Amazon Lookout for Equipment. </p>
    #[doc(hidden)]
    pub server_side_kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>Indicates that the asset associated with this sensor has been shut off. As long as this condition is met, Lookout for Equipment will not use data from this asset for training, evaluation, or inference.</p>
    #[doc(hidden)]
    pub off_condition: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeModelOutput {
    /// <p>The name of the ML model being described. </p>
    pub fn model_name(&self) -> ::std::option::Option<&str> {
        self.model_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the ML model being described. </p>
    pub fn model_arn(&self) -> ::std::option::Option<&str> {
        self.model_arn.as_deref()
    }
    /// <p>The name of the dataset being used by the ML being described. </p>
    pub fn dataset_name(&self) -> ::std::option::Option<&str> {
        self.dataset_name.as_deref()
    }
    /// <p>The Amazon Resouce Name (ARN) of the dataset used to create the ML model being described. </p>
    pub fn dataset_arn(&self) -> ::std::option::Option<&str> {
        self.dataset_arn.as_deref()
    }
    /// <p>A JSON description of the data that is in each time series dataset, including names, column names, and data types. </p>
    pub fn schema(&self) -> ::std::option::Option<&str> {
        self.schema.as_deref()
    }
    /// <p>Specifies configuration information about the labels input, including its S3 location. </p>
    pub fn labels_input_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::LabelsInputConfiguration> {
        self.labels_input_configuration.as_ref()
    }
    /// <p> Indicates the time reference in the dataset that was used to begin the subset of training data for the ML model. </p>
    pub fn training_data_start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.training_data_start_time.as_ref()
    }
    /// <p> Indicates the time reference in the dataset that was used to end the subset of training data for the ML model. </p>
    pub fn training_data_end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.training_data_end_time.as_ref()
    }
    /// <p> Indicates the time reference in the dataset that was used to begin the subset of evaluation data for the ML model. </p>
    pub fn evaluation_data_start_time(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.evaluation_data_start_time.as_ref()
    }
    /// <p> Indicates the time reference in the dataset that was used to end the subset of evaluation data for the ML model. </p>
    pub fn evaluation_data_end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.evaluation_data_end_time.as_ref()
    }
    /// <p> The Amazon Resource Name (ARN) of a role with permission to access the data source for the ML model being described. </p>
    pub fn role_arn(&self) -> ::std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The configuration is the <code>TargetSamplingRate</code>, which is the sampling rate of the data after post processing by Amazon Lookout for Equipment. For example, if you provide data that has been collected at a 1 second level and you want the system to resample the data at a 1 minute rate before training, the <code>TargetSamplingRate</code> is 1 minute.</p>
    /// <p>When providing a value for the <code>TargetSamplingRate</code>, you must attach the prefix "PT" to the rate you want. The value for a 1 second rate is therefore <i>PT1S</i>, the value for a 15 minute rate is <i>PT15M</i>, and the value for a 1 hour rate is <i>PT1H</i> </p>
    pub fn data_pre_processing_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::DataPreProcessingConfiguration> {
        self.data_pre_processing_configuration.as_ref()
    }
    /// <p>Specifies the current status of the model being described. Status describes the status of the most recent action of the model. </p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::ModelStatus> {
        self.status.as_ref()
    }
    /// <p>Indicates the time at which the training of the ML model began. </p>
    pub fn training_execution_start_time(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.training_execution_start_time.as_ref()
    }
    /// <p>Indicates the time at which the training of the ML model was completed. </p>
    pub fn training_execution_end_time(
        &self,
    ) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.training_execution_end_time.as_ref()
    }
    /// <p>If the training of the ML model failed, this indicates the reason for that failure. </p>
    pub fn failed_reason(&self) -> ::std::option::Option<&str> {
        self.failed_reason.as_deref()
    }
    /// <p>The Model Metrics show an aggregated summary of the model's performance within the evaluation time range. This is the JSON content of the metrics created when evaluating the model. </p>
    pub fn model_metrics(&self) -> ::std::option::Option<&str> {
        self.model_metrics.as_deref()
    }
    /// <p>Indicates the last time the ML model was updated. The type of update is not specified. </p>
    pub fn last_updated_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_time.as_ref()
    }
    /// <p>Indicates the time and date at which the ML model was created. </p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>Provides the identifier of the KMS key used to encrypt model data by Amazon Lookout for Equipment. </p>
    pub fn server_side_kms_key_id(&self) -> ::std::option::Option<&str> {
        self.server_side_kms_key_id.as_deref()
    }
    /// <p>Indicates that the asset associated with this sensor has been shut off. As long as this condition is met, Lookout for Equipment will not use data from this asset for training, evaluation, or inference.</p>
    pub fn off_condition(&self) -> ::std::option::Option<&str> {
        self.off_condition.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeModelOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeModelOutput {
    /// Creates a new builder-style object to manufacture [`DescribeModelOutput`](crate::operation::describe_model::DescribeModelOutput).
    pub fn builder() -> crate::operation::describe_model::builders::DescribeModelOutputBuilder {
        crate::operation::describe_model::builders::DescribeModelOutputBuilder::default()
    }
}

/// A builder for [`DescribeModelOutput`](crate::operation::describe_model::DescribeModelOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeModelOutputBuilder {
    pub(crate) model_name: ::std::option::Option<::std::string::String>,
    pub(crate) model_arn: ::std::option::Option<::std::string::String>,
    pub(crate) dataset_name: ::std::option::Option<::std::string::String>,
    pub(crate) dataset_arn: ::std::option::Option<::std::string::String>,
    pub(crate) schema: ::std::option::Option<::std::string::String>,
    pub(crate) labels_input_configuration:
        ::std::option::Option<crate::types::LabelsInputConfiguration>,
    pub(crate) training_data_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) training_data_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) evaluation_data_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) evaluation_data_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) data_pre_processing_configuration:
        ::std::option::Option<crate::types::DataPreProcessingConfiguration>,
    pub(crate) status: ::std::option::Option<crate::types::ModelStatus>,
    pub(crate) training_execution_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) training_execution_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) failed_reason: ::std::option::Option<::std::string::String>,
    pub(crate) model_metrics: ::std::option::Option<::std::string::String>,
    pub(crate) last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) server_side_kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) off_condition: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeModelOutputBuilder {
    /// <p>The name of the ML model being described. </p>
    pub fn model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the ML model being described. </p>
    pub fn set_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the ML model being described. </p>
    pub fn model_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the ML model being described. </p>
    pub fn set_model_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_arn = input;
        self
    }
    /// <p>The name of the dataset being used by the ML being described. </p>
    pub fn dataset_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dataset_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the dataset being used by the ML being described. </p>
    pub fn set_dataset_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dataset_name = input;
        self
    }
    /// <p>The Amazon Resouce Name (ARN) of the dataset used to create the ML model being described. </p>
    pub fn dataset_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dataset_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resouce Name (ARN) of the dataset used to create the ML model being described. </p>
    pub fn set_dataset_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dataset_arn = input;
        self
    }
    /// <p>A JSON description of the data that is in each time series dataset, including names, column names, and data types. </p>
    pub fn schema(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.schema = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A JSON description of the data that is in each time series dataset, including names, column names, and data types. </p>
    pub fn set_schema(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.schema = input;
        self
    }
    /// <p>Specifies configuration information about the labels input, including its S3 location. </p>
    pub fn labels_input_configuration(
        mut self,
        input: crate::types::LabelsInputConfiguration,
    ) -> Self {
        self.labels_input_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies configuration information about the labels input, including its S3 location. </p>
    pub fn set_labels_input_configuration(
        mut self,
        input: ::std::option::Option<crate::types::LabelsInputConfiguration>,
    ) -> Self {
        self.labels_input_configuration = input;
        self
    }
    /// <p> Indicates the time reference in the dataset that was used to begin the subset of training data for the ML model. </p>
    pub fn training_data_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.training_data_start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates the time reference in the dataset that was used to begin the subset of training data for the ML model. </p>
    pub fn set_training_data_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.training_data_start_time = input;
        self
    }
    /// <p> Indicates the time reference in the dataset that was used to end the subset of training data for the ML model. </p>
    pub fn training_data_end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.training_data_end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates the time reference in the dataset that was used to end the subset of training data for the ML model. </p>
    pub fn set_training_data_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.training_data_end_time = input;
        self
    }
    /// <p> Indicates the time reference in the dataset that was used to begin the subset of evaluation data for the ML model. </p>
    pub fn evaluation_data_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.evaluation_data_start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates the time reference in the dataset that was used to begin the subset of evaluation data for the ML model. </p>
    pub fn set_evaluation_data_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.evaluation_data_start_time = input;
        self
    }
    /// <p> Indicates the time reference in the dataset that was used to end the subset of evaluation data for the ML model. </p>
    pub fn evaluation_data_end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.evaluation_data_end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p> Indicates the time reference in the dataset that was used to end the subset of evaluation data for the ML model. </p>
    pub fn set_evaluation_data_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.evaluation_data_end_time = input;
        self
    }
    /// <p> The Amazon Resource Name (ARN) of a role with permission to access the data source for the ML model being described. </p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The Amazon Resource Name (ARN) of a role with permission to access the data source for the ML model being described. </p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The configuration is the <code>TargetSamplingRate</code>, which is the sampling rate of the data after post processing by Amazon Lookout for Equipment. For example, if you provide data that has been collected at a 1 second level and you want the system to resample the data at a 1 minute rate before training, the <code>TargetSamplingRate</code> is 1 minute.</p>
    /// <p>When providing a value for the <code>TargetSamplingRate</code>, you must attach the prefix "PT" to the rate you want. The value for a 1 second rate is therefore <i>PT1S</i>, the value for a 15 minute rate is <i>PT15M</i>, and the value for a 1 hour rate is <i>PT1H</i> </p>
    pub fn data_pre_processing_configuration(
        mut self,
        input: crate::types::DataPreProcessingConfiguration,
    ) -> Self {
        self.data_pre_processing_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration is the <code>TargetSamplingRate</code>, which is the sampling rate of the data after post processing by Amazon Lookout for Equipment. For example, if you provide data that has been collected at a 1 second level and you want the system to resample the data at a 1 minute rate before training, the <code>TargetSamplingRate</code> is 1 minute.</p>
    /// <p>When providing a value for the <code>TargetSamplingRate</code>, you must attach the prefix "PT" to the rate you want. The value for a 1 second rate is therefore <i>PT1S</i>, the value for a 15 minute rate is <i>PT15M</i>, and the value for a 1 hour rate is <i>PT1H</i> </p>
    pub fn set_data_pre_processing_configuration(
        mut self,
        input: ::std::option::Option<crate::types::DataPreProcessingConfiguration>,
    ) -> Self {
        self.data_pre_processing_configuration = input;
        self
    }
    /// <p>Specifies the current status of the model being described. Status describes the status of the most recent action of the model. </p>
    pub fn status(mut self, input: crate::types::ModelStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the current status of the model being described. Status describes the status of the most recent action of the model. </p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ModelStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>Indicates the time at which the training of the ML model began. </p>
    pub fn training_execution_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.training_execution_start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the time at which the training of the ML model began. </p>
    pub fn set_training_execution_start_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.training_execution_start_time = input;
        self
    }
    /// <p>Indicates the time at which the training of the ML model was completed. </p>
    pub fn training_execution_end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.training_execution_end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the time at which the training of the ML model was completed. </p>
    pub fn set_training_execution_end_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.training_execution_end_time = input;
        self
    }
    /// <p>If the training of the ML model failed, this indicates the reason for that failure. </p>
    pub fn failed_reason(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.failed_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the training of the ML model failed, this indicates the reason for that failure. </p>
    pub fn set_failed_reason(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.failed_reason = input;
        self
    }
    /// <p>The Model Metrics show an aggregated summary of the model's performance within the evaluation time range. This is the JSON content of the metrics created when evaluating the model. </p>
    pub fn model_metrics(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.model_metrics = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Model Metrics show an aggregated summary of the model's performance within the evaluation time range. This is the JSON content of the metrics created when evaluating the model. </p>
    pub fn set_model_metrics(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.model_metrics = input;
        self
    }
    /// <p>Indicates the last time the ML model was updated. The type of update is not specified. </p>
    pub fn last_updated_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the last time the ML model was updated. The type of update is not specified. </p>
    pub fn set_last_updated_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_time = input;
        self
    }
    /// <p>Indicates the time and date at which the ML model was created. </p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the time and date at which the ML model was created. </p>
    pub fn set_created_at(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.created_at = input;
        self
    }
    /// <p>Provides the identifier of the KMS key used to encrypt model data by Amazon Lookout for Equipment. </p>
    pub fn server_side_kms_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.server_side_kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Provides the identifier of the KMS key used to encrypt model data by Amazon Lookout for Equipment. </p>
    pub fn set_server_side_kms_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.server_side_kms_key_id = input;
        self
    }
    /// <p>Indicates that the asset associated with this sensor has been shut off. As long as this condition is met, Lookout for Equipment will not use data from this asset for training, evaluation, or inference.</p>
    pub fn off_condition(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.off_condition = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Indicates that the asset associated with this sensor has been shut off. As long as this condition is met, Lookout for Equipment will not use data from this asset for training, evaluation, or inference.</p>
    pub fn set_off_condition(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.off_condition = input;
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
    /// Consumes the builder and constructs a [`DescribeModelOutput`](crate::operation::describe_model::DescribeModelOutput).
    pub fn build(self) -> crate::operation::describe_model::DescribeModelOutput {
        crate::operation::describe_model::DescribeModelOutput {
            model_name: self.model_name,
            model_arn: self.model_arn,
            dataset_name: self.dataset_name,
            dataset_arn: self.dataset_arn,
            schema: self.schema,
            labels_input_configuration: self.labels_input_configuration,
            training_data_start_time: self.training_data_start_time,
            training_data_end_time: self.training_data_end_time,
            evaluation_data_start_time: self.evaluation_data_start_time,
            evaluation_data_end_time: self.evaluation_data_end_time,
            role_arn: self.role_arn,
            data_pre_processing_configuration: self.data_pre_processing_configuration,
            status: self.status,
            training_execution_start_time: self.training_execution_start_time,
            training_execution_end_time: self.training_execution_end_time,
            failed_reason: self.failed_reason,
            model_metrics: self.model_metrics,
            last_updated_time: self.last_updated_time,
            created_at: self.created_at,
            server_side_kms_key_id: self.server_side_kms_key_id,
            off_condition: self.off_condition,
            _request_id: self._request_id,
        }
    }
}
