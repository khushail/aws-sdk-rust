// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents a virtual router returned by a describe operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VirtualRouterData {
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    #[doc(hidden)]
    pub mesh_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the virtual router.</p>
    #[doc(hidden)]
    pub virtual_router_name: ::std::option::Option<::std::string::String>,
    /// <p>The specifications of the virtual router.</p>
    #[doc(hidden)]
    pub spec: ::std::option::Option<crate::types::VirtualRouterSpec>,
    /// <p>The associated metadata for the virtual router.</p>
    #[doc(hidden)]
    pub metadata: ::std::option::Option<crate::types::ResourceMetadata>,
    /// <p>The current status of the virtual router.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::VirtualRouterStatus>,
}
impl VirtualRouterData {
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    pub fn mesh_name(&self) -> ::std::option::Option<&str> {
        self.mesh_name.as_deref()
    }
    /// <p>The name of the virtual router.</p>
    pub fn virtual_router_name(&self) -> ::std::option::Option<&str> {
        self.virtual_router_name.as_deref()
    }
    /// <p>The specifications of the virtual router.</p>
    pub fn spec(&self) -> ::std::option::Option<&crate::types::VirtualRouterSpec> {
        self.spec.as_ref()
    }
    /// <p>The associated metadata for the virtual router.</p>
    pub fn metadata(&self) -> ::std::option::Option<&crate::types::ResourceMetadata> {
        self.metadata.as_ref()
    }
    /// <p>The current status of the virtual router.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::VirtualRouterStatus> {
        self.status.as_ref()
    }
}
impl VirtualRouterData {
    /// Creates a new builder-style object to manufacture [`VirtualRouterData`](crate::types::VirtualRouterData).
    pub fn builder() -> crate::types::builders::VirtualRouterDataBuilder {
        crate::types::builders::VirtualRouterDataBuilder::default()
    }
}

/// A builder for [`VirtualRouterData`](crate::types::VirtualRouterData).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VirtualRouterDataBuilder {
    pub(crate) mesh_name: ::std::option::Option<::std::string::String>,
    pub(crate) virtual_router_name: ::std::option::Option<::std::string::String>,
    pub(crate) spec: ::std::option::Option<crate::types::VirtualRouterSpec>,
    pub(crate) metadata: ::std::option::Option<crate::types::ResourceMetadata>,
    pub(crate) status: ::std::option::Option<crate::types::VirtualRouterStatus>,
}
impl VirtualRouterDataBuilder {
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    pub fn mesh_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mesh_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service mesh that the virtual router resides in.</p>
    pub fn set_mesh_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mesh_name = input;
        self
    }
    /// <p>The name of the virtual router.</p>
    pub fn virtual_router_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.virtual_router_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the virtual router.</p>
    pub fn set_virtual_router_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.virtual_router_name = input;
        self
    }
    /// <p>The specifications of the virtual router.</p>
    pub fn spec(mut self, input: crate::types::VirtualRouterSpec) -> Self {
        self.spec = ::std::option::Option::Some(input);
        self
    }
    /// <p>The specifications of the virtual router.</p>
    pub fn set_spec(
        mut self,
        input: ::std::option::Option<crate::types::VirtualRouterSpec>,
    ) -> Self {
        self.spec = input;
        self
    }
    /// <p>The associated metadata for the virtual router.</p>
    pub fn metadata(mut self, input: crate::types::ResourceMetadata) -> Self {
        self.metadata = ::std::option::Option::Some(input);
        self
    }
    /// <p>The associated metadata for the virtual router.</p>
    pub fn set_metadata(
        mut self,
        input: ::std::option::Option<crate::types::ResourceMetadata>,
    ) -> Self {
        self.metadata = input;
        self
    }
    /// <p>The current status of the virtual router.</p>
    pub fn status(mut self, input: crate::types::VirtualRouterStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the virtual router.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::VirtualRouterStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// Consumes the builder and constructs a [`VirtualRouterData`](crate::types::VirtualRouterData).
    pub fn build(self) -> crate::types::VirtualRouterData {
        crate::types::VirtualRouterData {
            mesh_name: self.mesh_name,
            virtual_router_name: self.virtual_router_name,
            spec: self.spec,
            metadata: self.metadata,
            status: self.status,
        }
    }
}
