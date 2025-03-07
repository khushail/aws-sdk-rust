// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The metadata of a network function.</p>
/// <p>A network instance is a single network created in Amazon Web Services TNB that can be deployed and on which life-cycle operations (like terminate, update, and delete) can be performed.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetSolVnfcResourceInfoMetadata {
    /// <p>Information about the node group.</p>
    #[doc(hidden)]
    pub node_group: ::std::option::Option<::std::string::String>,
    /// <p>Information about the cluster.</p>
    #[doc(hidden)]
    pub cluster: ::std::option::Option<::std::string::String>,
    /// <p>Information about the helm chart.</p>
    #[doc(hidden)]
    pub helm_chart: ::std::option::Option<::std::string::String>,
}
impl GetSolVnfcResourceInfoMetadata {
    /// <p>Information about the node group.</p>
    pub fn node_group(&self) -> ::std::option::Option<&str> {
        self.node_group.as_deref()
    }
    /// <p>Information about the cluster.</p>
    pub fn cluster(&self) -> ::std::option::Option<&str> {
        self.cluster.as_deref()
    }
    /// <p>Information about the helm chart.</p>
    pub fn helm_chart(&self) -> ::std::option::Option<&str> {
        self.helm_chart.as_deref()
    }
}
impl GetSolVnfcResourceInfoMetadata {
    /// Creates a new builder-style object to manufacture [`GetSolVnfcResourceInfoMetadata`](crate::types::GetSolVnfcResourceInfoMetadata).
    pub fn builder() -> crate::types::builders::GetSolVnfcResourceInfoMetadataBuilder {
        crate::types::builders::GetSolVnfcResourceInfoMetadataBuilder::default()
    }
}

/// A builder for [`GetSolVnfcResourceInfoMetadata`](crate::types::GetSolVnfcResourceInfoMetadata).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetSolVnfcResourceInfoMetadataBuilder {
    pub(crate) node_group: ::std::option::Option<::std::string::String>,
    pub(crate) cluster: ::std::option::Option<::std::string::String>,
    pub(crate) helm_chart: ::std::option::Option<::std::string::String>,
}
impl GetSolVnfcResourceInfoMetadataBuilder {
    /// <p>Information about the node group.</p>
    pub fn node_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.node_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Information about the node group.</p>
    pub fn set_node_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.node_group = input;
        self
    }
    /// <p>Information about the cluster.</p>
    pub fn cluster(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Information about the cluster.</p>
    pub fn set_cluster(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster = input;
        self
    }
    /// <p>Information about the helm chart.</p>
    pub fn helm_chart(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.helm_chart = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Information about the helm chart.</p>
    pub fn set_helm_chart(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.helm_chart = input;
        self
    }
    /// Consumes the builder and constructs a [`GetSolVnfcResourceInfoMetadata`](crate::types::GetSolVnfcResourceInfoMetadata).
    pub fn build(self) -> crate::types::GetSolVnfcResourceInfoMetadata {
        crate::types::GetSolVnfcResourceInfoMetadata {
            node_group: self.node_group,
            cluster: self.cluster,
            helm_chart: self.helm_chart,
        }
    }
}
