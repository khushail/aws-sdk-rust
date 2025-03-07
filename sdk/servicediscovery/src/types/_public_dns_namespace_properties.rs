// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>DNS properties for the public DNS namespace.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PublicDnsNamespaceProperties {
    /// <p>DNS properties for the public DNS namespace.</p>
    #[doc(hidden)]
    pub dns_properties: ::std::option::Option<crate::types::PublicDnsPropertiesMutable>,
}
impl PublicDnsNamespaceProperties {
    /// <p>DNS properties for the public DNS namespace.</p>
    pub fn dns_properties(
        &self,
    ) -> ::std::option::Option<&crate::types::PublicDnsPropertiesMutable> {
        self.dns_properties.as_ref()
    }
}
impl PublicDnsNamespaceProperties {
    /// Creates a new builder-style object to manufacture [`PublicDnsNamespaceProperties`](crate::types::PublicDnsNamespaceProperties).
    pub fn builder() -> crate::types::builders::PublicDnsNamespacePropertiesBuilder {
        crate::types::builders::PublicDnsNamespacePropertiesBuilder::default()
    }
}

/// A builder for [`PublicDnsNamespaceProperties`](crate::types::PublicDnsNamespaceProperties).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct PublicDnsNamespacePropertiesBuilder {
    pub(crate) dns_properties: ::std::option::Option<crate::types::PublicDnsPropertiesMutable>,
}
impl PublicDnsNamespacePropertiesBuilder {
    /// <p>DNS properties for the public DNS namespace.</p>
    pub fn dns_properties(mut self, input: crate::types::PublicDnsPropertiesMutable) -> Self {
        self.dns_properties = ::std::option::Option::Some(input);
        self
    }
    /// <p>DNS properties for the public DNS namespace.</p>
    pub fn set_dns_properties(
        mut self,
        input: ::std::option::Option<crate::types::PublicDnsPropertiesMutable>,
    ) -> Self {
        self.dns_properties = input;
        self
    }
    /// Consumes the builder and constructs a [`PublicDnsNamespaceProperties`](crate::types::PublicDnsNamespaceProperties).
    pub fn build(self) -> crate::types::PublicDnsNamespaceProperties {
        crate::types::PublicDnsNamespaceProperties {
            dns_properties: self.dns_properties,
        }
    }
}
