// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>For a SQL-based Kinesis Data Analytics application, provides the configuration information about an input processor. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">Amazon Lambda</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InputProcessingConfigurationDescription {
    /// <p>Provides configuration information about the associated <code>InputLambdaProcessorDescription</code> </p>
    #[doc(hidden)]
    pub input_lambda_processor_description:
        ::std::option::Option<crate::types::InputLambdaProcessorDescription>,
}
impl InputProcessingConfigurationDescription {
    /// <p>Provides configuration information about the associated <code>InputLambdaProcessorDescription</code> </p>
    pub fn input_lambda_processor_description(
        &self,
    ) -> ::std::option::Option<&crate::types::InputLambdaProcessorDescription> {
        self.input_lambda_processor_description.as_ref()
    }
}
impl InputProcessingConfigurationDescription {
    /// Creates a new builder-style object to manufacture [`InputProcessingConfigurationDescription`](crate::types::InputProcessingConfigurationDescription).
    pub fn builder() -> crate::types::builders::InputProcessingConfigurationDescriptionBuilder {
        crate::types::builders::InputProcessingConfigurationDescriptionBuilder::default()
    }
}

/// A builder for [`InputProcessingConfigurationDescription`](crate::types::InputProcessingConfigurationDescription).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct InputProcessingConfigurationDescriptionBuilder {
    pub(crate) input_lambda_processor_description:
        ::std::option::Option<crate::types::InputLambdaProcessorDescription>,
}
impl InputProcessingConfigurationDescriptionBuilder {
    /// <p>Provides configuration information about the associated <code>InputLambdaProcessorDescription</code> </p>
    pub fn input_lambda_processor_description(
        mut self,
        input: crate::types::InputLambdaProcessorDescription,
    ) -> Self {
        self.input_lambda_processor_description = ::std::option::Option::Some(input);
        self
    }
    /// <p>Provides configuration information about the associated <code>InputLambdaProcessorDescription</code> </p>
    pub fn set_input_lambda_processor_description(
        mut self,
        input: ::std::option::Option<crate::types::InputLambdaProcessorDescription>,
    ) -> Self {
        self.input_lambda_processor_description = input;
        self
    }
    /// Consumes the builder and constructs a [`InputProcessingConfigurationDescription`](crate::types::InputProcessingConfigurationDescription).
    pub fn build(self) -> crate::types::InputProcessingConfigurationDescription {
        crate::types::InputProcessingConfigurationDescription {
            input_lambda_processor_description: self.input_lambda_processor_description,
        }
    }
}
