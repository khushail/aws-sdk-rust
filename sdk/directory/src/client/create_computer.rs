// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateComputer`](crate::operation::create_computer::builders::CreateComputerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl ::std::convert::Into<String>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::set_directory_id): <p>The identifier of the directory in which to create the computer account.</p>
    ///   - [`computer_name(impl ::std::convert::Into<String>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::computer_name) / [`set_computer_name(Option<String>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::set_computer_name): <p>The name of the computer account.</p>
    ///   - [`password(impl ::std::convert::Into<String>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::password) / [`set_password(Option<String>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::set_password): <p>A one-time password that is used to join the computer to the directory. You should generate a random, strong password to use for this parameter.</p>
    ///   - [`organizational_unit_distinguished_name(impl ::std::convert::Into<String>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::organizational_unit_distinguished_name) / [`set_organizational_unit_distinguished_name(Option<String>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::set_organizational_unit_distinguished_name): <p>The fully-qualified distinguished name of the organizational unit to place the computer account in.</p>
    ///   - [`computer_attributes(Vec<Attribute>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::computer_attributes) / [`set_computer_attributes(Option<Vec<Attribute>>)`](crate::operation::create_computer::builders::CreateComputerFluentBuilder::set_computer_attributes): <p>An array of <code>Attribute</code> objects that contain any LDAP attributes to apply to the computer account.</p>
    /// - On success, responds with [`CreateComputerOutput`](crate::operation::create_computer::CreateComputerOutput) with field(s):
    ///   - [`computer(Option<Computer>)`](crate::operation::create_computer::CreateComputerOutput::computer): <p>A <code>Computer</code> object that represents the computer account.</p>
    /// - On failure, responds with [`SdkError<CreateComputerError>`](crate::operation::create_computer::CreateComputerError)
    pub fn create_computer(
        &self,
    ) -> crate::operation::create_computer::builders::CreateComputerFluentBuilder {
        crate::operation::create_computer::builders::CreateComputerFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
