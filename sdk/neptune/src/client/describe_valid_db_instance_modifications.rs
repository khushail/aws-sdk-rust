// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeValidDBInstanceModifications`](crate::operation::describe_valid_db_instance_modifications::builders::DescribeValidDBInstanceModificationsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`db_instance_identifier(impl ::std::convert::Into<String>)`](crate::operation::describe_valid_db_instance_modifications::builders::DescribeValidDBInstanceModificationsFluentBuilder::db_instance_identifier) / [`set_db_instance_identifier(Option<String>)`](crate::operation::describe_valid_db_instance_modifications::builders::DescribeValidDBInstanceModificationsFluentBuilder::set_db_instance_identifier): <p>The customer identifier or the ARN of your DB instance.</p>
    /// - On success, responds with [`DescribeValidDbInstanceModificationsOutput`](crate::operation::describe_valid_db_instance_modifications::DescribeValidDbInstanceModificationsOutput) with field(s):
    ///   - [`valid_db_instance_modifications_message(Option<ValidDbInstanceModificationsMessage>)`](crate::operation::describe_valid_db_instance_modifications::DescribeValidDbInstanceModificationsOutput::valid_db_instance_modifications_message): <p>Information about valid modifications that you can make to your DB instance. Contains the result of a successful call to the <code>DescribeValidDBInstanceModifications</code> action. You can use this information when you call <code>ModifyDBInstance</code>. </p>
    /// - On failure, responds with [`SdkError<DescribeValidDBInstanceModificationsError>`](crate::operation::describe_valid_db_instance_modifications::DescribeValidDBInstanceModificationsError)
    pub fn describe_valid_db_instance_modifications(&self) -> crate::operation::describe_valid_db_instance_modifications::builders::DescribeValidDBInstanceModificationsFluentBuilder{
        crate::operation::describe_valid_db_instance_modifications::builders::DescribeValidDBInstanceModificationsFluentBuilder::new(self.handle.clone())
    }
}
