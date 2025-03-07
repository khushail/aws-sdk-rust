// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVirtualServiceInput {
    /// <p>The name to use for the virtual service.</p>
    #[doc(hidden)]
    pub virtual_service_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the service mesh to create the virtual service in.</p>
    #[doc(hidden)]
    pub mesh_name: ::std::option::Option<::std::string::String>,
    /// <p>The virtual service specification to apply.</p>
    #[doc(hidden)]
    pub spec: ::std::option::Option<crate::types::VirtualServiceSpec>,
    /// <p>Optional metadata that you can apply to the virtual service to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::TagRef>>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    #[doc(hidden)]
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    #[doc(hidden)]
    pub mesh_owner: ::std::option::Option<::std::string::String>,
}
impl CreateVirtualServiceInput {
    /// <p>The name to use for the virtual service.</p>
    pub fn virtual_service_name(&self) -> ::std::option::Option<&str> {
        self.virtual_service_name.as_deref()
    }
    /// <p>The name of the service mesh to create the virtual service in.</p>
    pub fn mesh_name(&self) -> ::std::option::Option<&str> {
        self.mesh_name.as_deref()
    }
    /// <p>The virtual service specification to apply.</p>
    pub fn spec(&self) -> ::std::option::Option<&crate::types::VirtualServiceSpec> {
        self.spec.as_ref()
    }
    /// <p>Optional metadata that you can apply to the virtual service to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::TagRef]> {
        self.tags.as_deref()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    pub fn mesh_owner(&self) -> ::std::option::Option<&str> {
        self.mesh_owner.as_deref()
    }
}
impl CreateVirtualServiceInput {
    /// Creates a new builder-style object to manufacture [`CreateVirtualServiceInput`](crate::operation::create_virtual_service::CreateVirtualServiceInput).
    pub fn builder(
    ) -> crate::operation::create_virtual_service::builders::CreateVirtualServiceInputBuilder {
        crate::operation::create_virtual_service::builders::CreateVirtualServiceInputBuilder::default()
    }
}

/// A builder for [`CreateVirtualServiceInput`](crate::operation::create_virtual_service::CreateVirtualServiceInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateVirtualServiceInputBuilder {
    pub(crate) virtual_service_name: ::std::option::Option<::std::string::String>,
    pub(crate) mesh_name: ::std::option::Option<::std::string::String>,
    pub(crate) spec: ::std::option::Option<crate::types::VirtualServiceSpec>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::TagRef>>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) mesh_owner: ::std::option::Option<::std::string::String>,
}
impl CreateVirtualServiceInputBuilder {
    /// <p>The name to use for the virtual service.</p>
    pub fn virtual_service_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.virtual_service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name to use for the virtual service.</p>
    pub fn set_virtual_service_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.virtual_service_name = input;
        self
    }
    /// <p>The name of the service mesh to create the virtual service in.</p>
    pub fn mesh_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mesh_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service mesh to create the virtual service in.</p>
    pub fn set_mesh_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mesh_name = input;
        self
    }
    /// <p>The virtual service specification to apply.</p>
    pub fn spec(mut self, input: crate::types::VirtualServiceSpec) -> Self {
        self.spec = ::std::option::Option::Some(input);
        self
    }
    /// <p>The virtual service specification to apply.</p>
    pub fn set_spec(
        mut self,
        input: ::std::option::Option<crate::types::VirtualServiceSpec>,
    ) -> Self {
        self.spec = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Optional metadata that you can apply to the virtual service to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    pub fn tags(mut self, input: crate::types::TagRef) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Optional metadata that you can apply to the virtual service to assist with categorization and organization. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TagRef>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 36 letters, numbers, hyphens, and underscores are allowed.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    pub fn mesh_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mesh_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services IAM account ID of the service mesh owner. If the account ID is not your own, then the account that you specify must share the mesh with your account before you can create the resource in the service mesh. For more information about mesh sharing, see <a href="https://docs.aws.amazon.com/app-mesh/latest/userguide/sharing.html">Working with shared meshes</a>.</p>
    pub fn set_mesh_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mesh_owner = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateVirtualServiceInput`](crate::operation::create_virtual_service::CreateVirtualServiceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_virtual_service::CreateVirtualServiceInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_virtual_service::CreateVirtualServiceInput {
                virtual_service_name: self.virtual_service_name,
                mesh_name: self.mesh_name,
                spec: self.spec,
                tags: self.tags,
                client_token: self.client_token,
                mesh_owner: self.mesh_owner,
            },
        )
    }
}
