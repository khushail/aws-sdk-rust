// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeFlywheel`](crate::operation::describe_flywheel::builders::DescribeFlywheelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`flywheel_arn(impl ::std::convert::Into<String>)`](crate::operation::describe_flywheel::builders::DescribeFlywheelFluentBuilder::flywheel_arn) / [`set_flywheel_arn(Option<String>)`](crate::operation::describe_flywheel::builders::DescribeFlywheelFluentBuilder::set_flywheel_arn): <p>The Amazon Resource Number (ARN) of the flywheel.</p>
    /// - On success, responds with [`DescribeFlywheelOutput`](crate::operation::describe_flywheel::DescribeFlywheelOutput) with field(s):
    ///   - [`flywheel_properties(Option<FlywheelProperties>)`](crate::operation::describe_flywheel::DescribeFlywheelOutput::flywheel_properties): <p>The flywheel properties.</p>
    /// - On failure, responds with [`SdkError<DescribeFlywheelError>`](crate::operation::describe_flywheel::DescribeFlywheelError)
    pub fn describe_flywheel(
        &self,
    ) -> crate::operation::describe_flywheel::builders::DescribeFlywheelFluentBuilder {
        crate::operation::describe_flywheel::builders::DescribeFlywheelFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
