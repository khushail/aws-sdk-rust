// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UntagResource`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl ::std::convert::Into<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_resource_arn): <p>Identifies the Application Auto Scaling scalable target from which to remove tags.</p>  <p>For example: <code>arn:aws:application-autoscaling:us-east-1:123456789012:scalable-target/1234abcd56ab78cd901ef1234567890ab123</code> </p>  <p>To get the ARN for a scalable target, use <code>DescribeScalableTargets</code>.</p>
    ///   - [`tag_keys(Vec<String>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::tag_keys) / [`set_tag_keys(Option<Vec<String>>)`](crate::operation::untag_resource::builders::UntagResourceFluentBuilder::set_tag_keys): <p>One or more tag keys. Specify only the tag keys, not the tag values.</p>
    /// - On success, responds with [`UntagResourceOutput`](crate::operation::untag_resource::UntagResourceOutput)
    /// - On failure, responds with [`SdkError<UntagResourceError>`](crate::operation::untag_resource::UntagResourceError)
    pub fn untag_resource(
        &self,
    ) -> crate::operation::untag_resource::builders::UntagResourceFluentBuilder {
        crate::operation::untag_resource::builders::UntagResourceFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
