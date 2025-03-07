// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeactivateAnomalyDetector`](crate::operation::deactivate_anomaly_detector::builders::DeactivateAnomalyDetectorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`anomaly_detector_arn(impl ::std::convert::Into<String>)`](crate::operation::deactivate_anomaly_detector::builders::DeactivateAnomalyDetectorFluentBuilder::anomaly_detector_arn) / [`set_anomaly_detector_arn(Option<String>)`](crate::operation::deactivate_anomaly_detector::builders::DeactivateAnomalyDetectorFluentBuilder::set_anomaly_detector_arn): <p>The Amazon Resource Name (ARN) of the anomaly detector.</p>
    /// - On success, responds with [`DeactivateAnomalyDetectorOutput`](crate::operation::deactivate_anomaly_detector::DeactivateAnomalyDetectorOutput)
    /// - On failure, responds with [`SdkError<DeactivateAnomalyDetectorError>`](crate::operation::deactivate_anomaly_detector::DeactivateAnomalyDetectorError)
    pub fn deactivate_anomaly_detector(&self) -> crate::operation::deactivate_anomaly_detector::builders::DeactivateAnomalyDetectorFluentBuilder{
        crate::operation::deactivate_anomaly_detector::builders::DeactivateAnomalyDetectorFluentBuilder::new(self.handle.clone())
    }
}
