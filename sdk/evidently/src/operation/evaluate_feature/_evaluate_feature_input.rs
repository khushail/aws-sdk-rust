// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EvaluateFeatureInput {
    /// <p>The name or ARN of the project that contains this feature.</p>
    #[doc(hidden)]
    pub project: ::std::option::Option<::std::string::String>,
    /// <p>The name of the feature being evaluated.</p>
    #[doc(hidden)]
    pub feature: ::std::option::Option<::std::string::String>,
    /// <p>An internal ID that represents a unique user of the application. This <code>entityID</code> is checked against any override rules assigned for this feature.</p>
    #[doc(hidden)]
    pub entity_id: ::std::option::Option<::std::string::String>,
    /// <p>A JSON object of attributes that you can optionally pass in as part of the evaluation event sent to Evidently from the user session. Evidently can use this value to match user sessions with defined audience segments. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html">Use segments to focus your audience</a>.</p>
    /// <p>If you include this parameter, the value must be a JSON object. A JSON array is not supported.</p>
    #[doc(hidden)]
    pub evaluation_context: ::std::option::Option<::std::string::String>,
}
impl EvaluateFeatureInput {
    /// <p>The name or ARN of the project that contains this feature.</p>
    pub fn project(&self) -> ::std::option::Option<&str> {
        self.project.as_deref()
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn feature(&self) -> ::std::option::Option<&str> {
        self.feature.as_deref()
    }
    /// <p>An internal ID that represents a unique user of the application. This <code>entityID</code> is checked against any override rules assigned for this feature.</p>
    pub fn entity_id(&self) -> ::std::option::Option<&str> {
        self.entity_id.as_deref()
    }
    /// <p>A JSON object of attributes that you can optionally pass in as part of the evaluation event sent to Evidently from the user session. Evidently can use this value to match user sessions with defined audience segments. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html">Use segments to focus your audience</a>.</p>
    /// <p>If you include this parameter, the value must be a JSON object. A JSON array is not supported.</p>
    pub fn evaluation_context(&self) -> ::std::option::Option<&str> {
        self.evaluation_context.as_deref()
    }
}
impl EvaluateFeatureInput {
    /// Creates a new builder-style object to manufacture [`EvaluateFeatureInput`](crate::operation::evaluate_feature::EvaluateFeatureInput).
    pub fn builder() -> crate::operation::evaluate_feature::builders::EvaluateFeatureInputBuilder {
        crate::operation::evaluate_feature::builders::EvaluateFeatureInputBuilder::default()
    }
}

/// A builder for [`EvaluateFeatureInput`](crate::operation::evaluate_feature::EvaluateFeatureInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EvaluateFeatureInputBuilder {
    pub(crate) project: ::std::option::Option<::std::string::String>,
    pub(crate) feature: ::std::option::Option<::std::string::String>,
    pub(crate) entity_id: ::std::option::Option<::std::string::String>,
    pub(crate) evaluation_context: ::std::option::Option<::std::string::String>,
}
impl EvaluateFeatureInputBuilder {
    /// <p>The name or ARN of the project that contains this feature.</p>
    pub fn project(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name or ARN of the project that contains this feature.</p>
    pub fn set_project(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project = input;
        self
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn feature(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.feature = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the feature being evaluated.</p>
    pub fn set_feature(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.feature = input;
        self
    }
    /// <p>An internal ID that represents a unique user of the application. This <code>entityID</code> is checked against any override rules assigned for this feature.</p>
    pub fn entity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.entity_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An internal ID that represents a unique user of the application. This <code>entityID</code> is checked against any override rules assigned for this feature.</p>
    pub fn set_entity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.entity_id = input;
        self
    }
    /// <p>A JSON object of attributes that you can optionally pass in as part of the evaluation event sent to Evidently from the user session. Evidently can use this value to match user sessions with defined audience segments. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html">Use segments to focus your audience</a>.</p>
    /// <p>If you include this parameter, the value must be a JSON object. A JSON array is not supported.</p>
    pub fn evaluation_context(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.evaluation_context = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A JSON object of attributes that you can optionally pass in as part of the evaluation event sent to Evidently from the user session. Evidently can use this value to match user sessions with defined audience segments. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch-Evidently-segments.html">Use segments to focus your audience</a>.</p>
    /// <p>If you include this parameter, the value must be a JSON object. A JSON array is not supported.</p>
    pub fn set_evaluation_context(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.evaluation_context = input;
        self
    }
    /// Consumes the builder and constructs a [`EvaluateFeatureInput`](crate::operation::evaluate_feature::EvaluateFeatureInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::evaluate_feature::EvaluateFeatureInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::evaluate_feature::EvaluateFeatureInput {
            project: self.project,
            feature: self.feature,
            entity_id: self.entity_id,
            evaluation_context: self.evaluation_context,
        })
    }
}
