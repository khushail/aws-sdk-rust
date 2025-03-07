// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAccountCustomization`](crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl ::std::convert::Into<String>)`](crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder::set_aws_account_id): <p>The ID for the Amazon Web Services account that you want to describe Amazon QuickSight customizations for.</p>
    ///   - [`namespace(impl ::std::convert::Into<String>)`](crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder::set_namespace): <p>The Amazon QuickSight namespace that you want to describe Amazon QuickSight customizations for.</p>
    ///   - [`resolved(bool)`](crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder::resolved) / [`set_resolved(Option<bool>)`](crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder::set_resolved): <p>The <code>Resolved</code> flag works with the other parameters to determine which view of Amazon QuickSight customizations is returned. You can add this flag to your command to use the same view that Amazon QuickSight uses to identify which customizations to apply to the console. Omit this flag, or set it to <code>no-resolved</code>, to reveal customizations that are configured at different levels. </p>
    /// - On success, responds with [`DescribeAccountCustomizationOutput`](crate::operation::describe_account_customization::DescribeAccountCustomizationOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::describe_account_customization::DescribeAccountCustomizationOutput::arn): <p>The Amazon Resource Name (ARN) of the customization that's associated with this Amazon Web Services account.</p>
    ///   - [`aws_account_id(Option<String>)`](crate::operation::describe_account_customization::DescribeAccountCustomizationOutput::aws_account_id): <p>The ID for the Amazon Web Services account that you're describing.</p>
    ///   - [`namespace(Option<String>)`](crate::operation::describe_account_customization::DescribeAccountCustomizationOutput::namespace): <p>The Amazon QuickSight namespace that you're describing. </p>
    ///   - [`account_customization(Option<AccountCustomization>)`](crate::operation::describe_account_customization::DescribeAccountCustomizationOutput::account_customization): <p>The Amazon QuickSight customizations that exist in the current Amazon Web Services Region. </p>
    ///   - [`request_id(Option<String>)`](crate::operation::describe_account_customization::DescribeAccountCustomizationOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    ///   - [`status(i32)`](crate::operation::describe_account_customization::DescribeAccountCustomizationOutput::status): <p>The HTTP status of the request.</p>
    /// - On failure, responds with [`SdkError<DescribeAccountCustomizationError>`](crate::operation::describe_account_customization::DescribeAccountCustomizationError)
    pub fn describe_account_customization(&self) -> crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder{
        crate::operation::describe_account_customization::builders::DescribeAccountCustomizationFluentBuilder::new(self.handle.clone())
    }
}
