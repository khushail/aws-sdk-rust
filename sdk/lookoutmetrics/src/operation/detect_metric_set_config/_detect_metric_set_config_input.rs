// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DetectMetricSetConfigInput {
    /// <p>An anomaly detector ARN.</p>
    #[doc(hidden)]
    pub anomaly_detector_arn: ::std::option::Option<::std::string::String>,
    /// <p>A data source.</p>
    #[doc(hidden)]
    pub auto_detection_metric_source:
        ::std::option::Option<crate::types::AutoDetectionMetricSource>,
}
impl DetectMetricSetConfigInput {
    /// <p>An anomaly detector ARN.</p>
    pub fn anomaly_detector_arn(&self) -> ::std::option::Option<&str> {
        self.anomaly_detector_arn.as_deref()
    }
    /// <p>A data source.</p>
    pub fn auto_detection_metric_source(
        &self,
    ) -> ::std::option::Option<&crate::types::AutoDetectionMetricSource> {
        self.auto_detection_metric_source.as_ref()
    }
}
impl DetectMetricSetConfigInput {
    /// Creates a new builder-style object to manufacture [`DetectMetricSetConfigInput`](crate::operation::detect_metric_set_config::DetectMetricSetConfigInput).
    pub fn builder(
    ) -> crate::operation::detect_metric_set_config::builders::DetectMetricSetConfigInputBuilder
    {
        crate::operation::detect_metric_set_config::builders::DetectMetricSetConfigInputBuilder::default()
    }
}

/// A builder for [`DetectMetricSetConfigInput`](crate::operation::detect_metric_set_config::DetectMetricSetConfigInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DetectMetricSetConfigInputBuilder {
    pub(crate) anomaly_detector_arn: ::std::option::Option<::std::string::String>,
    pub(crate) auto_detection_metric_source:
        ::std::option::Option<crate::types::AutoDetectionMetricSource>,
}
impl DetectMetricSetConfigInputBuilder {
    /// <p>An anomaly detector ARN.</p>
    pub fn anomaly_detector_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.anomaly_detector_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An anomaly detector ARN.</p>
    pub fn set_anomaly_detector_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.anomaly_detector_arn = input;
        self
    }
    /// <p>A data source.</p>
    pub fn auto_detection_metric_source(
        mut self,
        input: crate::types::AutoDetectionMetricSource,
    ) -> Self {
        self.auto_detection_metric_source = ::std::option::Option::Some(input);
        self
    }
    /// <p>A data source.</p>
    pub fn set_auto_detection_metric_source(
        mut self,
        input: ::std::option::Option<crate::types::AutoDetectionMetricSource>,
    ) -> Self {
        self.auto_detection_metric_source = input;
        self
    }
    /// Consumes the builder and constructs a [`DetectMetricSetConfigInput`](crate::operation::detect_metric_set_config::DetectMetricSetConfigInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::detect_metric_set_config::DetectMetricSetConfigInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::detect_metric_set_config::DetectMetricSetConfigInput {
                anomaly_detector_arn: self.anomaly_detector_arn,
                auto_detection_metric_source: self.auto_detection_metric_source,
            },
        )
    }
}
