// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateApprovedOrigin`](crate::operation::associate_approved_origin::builders::AssociateApprovedOriginFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl ::std::convert::Into<String>)`](crate::operation::associate_approved_origin::builders::AssociateApprovedOriginFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::associate_approved_origin::builders::AssociateApprovedOriginFluentBuilder::set_instance_id): <p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p>
    ///   - [`origin(impl ::std::convert::Into<String>)`](crate::operation::associate_approved_origin::builders::AssociateApprovedOriginFluentBuilder::origin) / [`set_origin(Option<String>)`](crate::operation::associate_approved_origin::builders::AssociateApprovedOriginFluentBuilder::set_origin): <p>The domain to add to your allow list.</p>
    /// - On success, responds with [`AssociateApprovedOriginOutput`](crate::operation::associate_approved_origin::AssociateApprovedOriginOutput)
    /// - On failure, responds with [`SdkError<AssociateApprovedOriginError>`](crate::operation::associate_approved_origin::AssociateApprovedOriginError)
    pub fn associate_approved_origin(
        &self,
    ) -> crate::operation::associate_approved_origin::builders::AssociateApprovedOriginFluentBuilder
    {
        crate::operation::associate_approved_origin::builders::AssociateApprovedOriginFluentBuilder::new(self.handle.clone())
    }
}
