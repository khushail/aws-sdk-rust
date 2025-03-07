// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdatePublishingDestination`](crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`detector_id(impl ::std::convert::Into<String>)`](crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder::detector_id) / [`set_detector_id(Option<String>)`](crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder::set_detector_id): <p>The ID of the detector associated with the publishing destinations to update.</p>
    ///   - [`destination_id(impl ::std::convert::Into<String>)`](crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder::destination_id) / [`set_destination_id(Option<String>)`](crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder::set_destination_id): <p>The ID of the publishing destination to update.</p>
    ///   - [`destination_properties(DestinationProperties)`](crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder::destination_properties) / [`set_destination_properties(Option<DestinationProperties>)`](crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder::set_destination_properties): <p>A <code>DestinationProperties</code> object that includes the <code>DestinationArn</code> and <code>KmsKeyArn</code> of the publishing destination.</p>
    /// - On success, responds with [`UpdatePublishingDestinationOutput`](crate::operation::update_publishing_destination::UpdatePublishingDestinationOutput)
    /// - On failure, responds with [`SdkError<UpdatePublishingDestinationError>`](crate::operation::update_publishing_destination::UpdatePublishingDestinationError)
    pub fn update_publishing_destination(&self) -> crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder{
        crate::operation::update_publishing_destination::builders::UpdatePublishingDestinationFluentBuilder::new(self.handle.clone())
    }
}
