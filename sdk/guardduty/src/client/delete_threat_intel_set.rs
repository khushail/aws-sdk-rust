// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteThreatIntelSet`](crate::operation::delete_threat_intel_set::builders::DeleteThreatIntelSetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`detector_id(impl ::std::convert::Into<String>)`](crate::operation::delete_threat_intel_set::builders::DeleteThreatIntelSetFluentBuilder::detector_id) / [`set_detector_id(Option<String>)`](crate::operation::delete_threat_intel_set::builders::DeleteThreatIntelSetFluentBuilder::set_detector_id): <p>The unique ID of the detector that the threatIntelSet is associated with.</p>
    ///   - [`threat_intel_set_id(impl ::std::convert::Into<String>)`](crate::operation::delete_threat_intel_set::builders::DeleteThreatIntelSetFluentBuilder::threat_intel_set_id) / [`set_threat_intel_set_id(Option<String>)`](crate::operation::delete_threat_intel_set::builders::DeleteThreatIntelSetFluentBuilder::set_threat_intel_set_id): <p>The unique ID of the threatIntelSet that you want to delete.</p>
    /// - On success, responds with [`DeleteThreatIntelSetOutput`](crate::operation::delete_threat_intel_set::DeleteThreatIntelSetOutput)
    /// - On failure, responds with [`SdkError<DeleteThreatIntelSetError>`](crate::operation::delete_threat_intel_set::DeleteThreatIntelSetError)
    pub fn delete_threat_intel_set(
        &self,
    ) -> crate::operation::delete_threat_intel_set::builders::DeleteThreatIntelSetFluentBuilder
    {
        crate::operation::delete_threat_intel_set::builders::DeleteThreatIntelSetFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
