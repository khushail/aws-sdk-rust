// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAddress`](crate::operation::describe_address::builders::DescribeAddressFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`address_id(impl ::std::convert::Into<String>)`](crate::operation::describe_address::builders::DescribeAddressFluentBuilder::address_id) / [`set_address_id(Option<String>)`](crate::operation::describe_address::builders::DescribeAddressFluentBuilder::set_address_id): <p>The automatically generated ID for a specific address.</p>
    /// - On success, responds with [`DescribeAddressOutput`](crate::operation::describe_address::DescribeAddressOutput) with field(s):
    ///   - [`address(Option<Address>)`](crate::operation::describe_address::DescribeAddressOutput::address): <p>The address that you want the Snow device(s) associated with a specific job to be shipped to.</p>
    /// - On failure, responds with [`SdkError<DescribeAddressError>`](crate::operation::describe_address::DescribeAddressError)
    pub fn describe_address(
        &self,
    ) -> crate::operation::describe_address::builders::DescribeAddressFluentBuilder {
        crate::operation::describe_address::builders::DescribeAddressFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
