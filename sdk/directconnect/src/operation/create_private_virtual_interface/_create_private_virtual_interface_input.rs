// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreatePrivateVirtualInterfaceInput {
    /// <p>The ID of the connection.</p>
    #[doc(hidden)]
    pub connection_id: ::std::option::Option<::std::string::String>,
    /// <p>Information about the private virtual interface.</p>
    #[doc(hidden)]
    pub new_private_virtual_interface:
        ::std::option::Option<crate::types::NewPrivateVirtualInterface>,
}
impl CreatePrivateVirtualInterfaceInput {
    /// <p>The ID of the connection.</p>
    pub fn connection_id(&self) -> ::std::option::Option<&str> {
        self.connection_id.as_deref()
    }
    /// <p>Information about the private virtual interface.</p>
    pub fn new_private_virtual_interface(
        &self,
    ) -> ::std::option::Option<&crate::types::NewPrivateVirtualInterface> {
        self.new_private_virtual_interface.as_ref()
    }
}
impl CreatePrivateVirtualInterfaceInput {
    /// Creates a new builder-style object to manufacture [`CreatePrivateVirtualInterfaceInput`](crate::operation::create_private_virtual_interface::CreatePrivateVirtualInterfaceInput).
    pub fn builder() -> crate::operation::create_private_virtual_interface::builders::CreatePrivateVirtualInterfaceInputBuilder{
        crate::operation::create_private_virtual_interface::builders::CreatePrivateVirtualInterfaceInputBuilder::default()
    }
}

/// A builder for [`CreatePrivateVirtualInterfaceInput`](crate::operation::create_private_virtual_interface::CreatePrivateVirtualInterfaceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreatePrivateVirtualInterfaceInputBuilder {
    pub(crate) connection_id: ::std::option::Option<::std::string::String>,
    pub(crate) new_private_virtual_interface:
        ::std::option::Option<crate::types::NewPrivateVirtualInterface>,
}
impl CreatePrivateVirtualInterfaceInputBuilder {
    /// <p>The ID of the connection.</p>
    pub fn connection_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the connection.</p>
    pub fn set_connection_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.connection_id = input;
        self
    }
    /// <p>Information about the private virtual interface.</p>
    pub fn new_private_virtual_interface(
        mut self,
        input: crate::types::NewPrivateVirtualInterface,
    ) -> Self {
        self.new_private_virtual_interface = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the private virtual interface.</p>
    pub fn set_new_private_virtual_interface(
        mut self,
        input: ::std::option::Option<crate::types::NewPrivateVirtualInterface>,
    ) -> Self {
        self.new_private_virtual_interface = input;
        self
    }
    /// Consumes the builder and constructs a [`CreatePrivateVirtualInterfaceInput`](crate::operation::create_private_virtual_interface::CreatePrivateVirtualInterfaceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_private_virtual_interface::CreatePrivateVirtualInterfaceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_private_virtual_interface::CreatePrivateVirtualInterfaceInput {
                connection_id: self.connection_id
                ,
                new_private_virtual_interface: self.new_private_virtual_interface
                ,
            }
        )
    }
}
