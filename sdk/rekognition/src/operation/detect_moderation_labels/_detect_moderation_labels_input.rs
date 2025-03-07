// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DetectModerationLabelsInput {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[doc(hidden)]
    pub image: ::std::option::Option<crate::types::Image>,
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence level lower than this specified value.</p>
    /// <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    #[doc(hidden)]
    pub min_confidence: ::std::option::Option<f32>,
    /// <p>Sets up the configuration for human evaluation, including the FlowDefinition the image will be sent to.</p>
    #[doc(hidden)]
    pub human_loop_config: ::std::option::Option<crate::types::HumanLoopConfig>,
}
impl DetectModerationLabelsInput {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn image(&self) -> ::std::option::Option<&crate::types::Image> {
        self.image.as_ref()
    }
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence level lower than this specified value.</p>
    /// <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    pub fn min_confidence(&self) -> ::std::option::Option<f32> {
        self.min_confidence
    }
    /// <p>Sets up the configuration for human evaluation, including the FlowDefinition the image will be sent to.</p>
    pub fn human_loop_config(&self) -> ::std::option::Option<&crate::types::HumanLoopConfig> {
        self.human_loop_config.as_ref()
    }
}
impl DetectModerationLabelsInput {
    /// Creates a new builder-style object to manufacture [`DetectModerationLabelsInput`](crate::operation::detect_moderation_labels::DetectModerationLabelsInput).
    pub fn builder(
    ) -> crate::operation::detect_moderation_labels::builders::DetectModerationLabelsInputBuilder
    {
        crate::operation::detect_moderation_labels::builders::DetectModerationLabelsInputBuilder::default()
    }
}

/// A builder for [`DetectModerationLabelsInput`](crate::operation::detect_moderation_labels::DetectModerationLabelsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DetectModerationLabelsInputBuilder {
    pub(crate) image: ::std::option::Option<crate::types::Image>,
    pub(crate) min_confidence: ::std::option::Option<f32>,
    pub(crate) human_loop_config: ::std::option::Option<crate::types::HumanLoopConfig>,
}
impl DetectModerationLabelsInputBuilder {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn image(mut self, input: crate::types::Image) -> Self {
        self.image = ::std::option::Option::Some(input);
        self
    }
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p>
    /// <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    pub fn set_image(mut self, input: ::std::option::Option<crate::types::Image>) -> Self {
        self.image = input;
        self
    }
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence level lower than this specified value.</p>
    /// <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    pub fn min_confidence(mut self, input: f32) -> Self {
        self.min_confidence = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence level lower than this specified value.</p>
    /// <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    pub fn set_min_confidence(mut self, input: ::std::option::Option<f32>) -> Self {
        self.min_confidence = input;
        self
    }
    /// <p>Sets up the configuration for human evaluation, including the FlowDefinition the image will be sent to.</p>
    pub fn human_loop_config(mut self, input: crate::types::HumanLoopConfig) -> Self {
        self.human_loop_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Sets up the configuration for human evaluation, including the FlowDefinition the image will be sent to.</p>
    pub fn set_human_loop_config(
        mut self,
        input: ::std::option::Option<crate::types::HumanLoopConfig>,
    ) -> Self {
        self.human_loop_config = input;
        self
    }
    /// Consumes the builder and constructs a [`DetectModerationLabelsInput`](crate::operation::detect_moderation_labels::DetectModerationLabelsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::detect_moderation_labels::DetectModerationLabelsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::detect_moderation_labels::DetectModerationLabelsInput {
                image: self.image,
                min_confidence: self.min_confidence,
                human_loop_config: self.human_loop_config,
            },
        )
    }
}
