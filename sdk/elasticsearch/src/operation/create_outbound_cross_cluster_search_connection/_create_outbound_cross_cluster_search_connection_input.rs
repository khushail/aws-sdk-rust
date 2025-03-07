// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container for the parameters to the <code><code>CreateOutboundCrossClusterSearchConnection</code></code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateOutboundCrossClusterSearchConnectionInput {
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the source Elasticsearch domain.</p>
    #[doc(hidden)]
    pub source_domain_info: ::std::option::Option<crate::types::DomainInformation>,
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the destination Elasticsearch domain.</p>
    #[doc(hidden)]
    pub destination_domain_info: ::std::option::Option<crate::types::DomainInformation>,
    /// <p>Specifies the connection alias that will be used by the customer for this connection.</p>
    #[doc(hidden)]
    pub connection_alias: ::std::option::Option<::std::string::String>,
}
impl CreateOutboundCrossClusterSearchConnectionInput {
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the source Elasticsearch domain.</p>
    pub fn source_domain_info(&self) -> ::std::option::Option<&crate::types::DomainInformation> {
        self.source_domain_info.as_ref()
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the destination Elasticsearch domain.</p>
    pub fn destination_domain_info(
        &self,
    ) -> ::std::option::Option<&crate::types::DomainInformation> {
        self.destination_domain_info.as_ref()
    }
    /// <p>Specifies the connection alias that will be used by the customer for this connection.</p>
    pub fn connection_alias(&self) -> ::std::option::Option<&str> {
        self.connection_alias.as_deref()
    }
}
impl CreateOutboundCrossClusterSearchConnectionInput {
    /// Creates a new builder-style object to manufacture [`CreateOutboundCrossClusterSearchConnectionInput`](crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionInput).
    pub fn builder() -> crate::operation::create_outbound_cross_cluster_search_connection::builders::CreateOutboundCrossClusterSearchConnectionInputBuilder{
        crate::operation::create_outbound_cross_cluster_search_connection::builders::CreateOutboundCrossClusterSearchConnectionInputBuilder::default()
    }
}

/// A builder for [`CreateOutboundCrossClusterSearchConnectionInput`](crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateOutboundCrossClusterSearchConnectionInputBuilder {
    pub(crate) source_domain_info: ::std::option::Option<crate::types::DomainInformation>,
    pub(crate) destination_domain_info: ::std::option::Option<crate::types::DomainInformation>,
    pub(crate) connection_alias: ::std::option::Option<::std::string::String>,
}
impl CreateOutboundCrossClusterSearchConnectionInputBuilder {
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the source Elasticsearch domain.</p>
    pub fn source_domain_info(mut self, input: crate::types::DomainInformation) -> Self {
        self.source_domain_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the source Elasticsearch domain.</p>
    pub fn set_source_domain_info(
        mut self,
        input: ::std::option::Option<crate::types::DomainInformation>,
    ) -> Self {
        self.source_domain_info = input;
        self
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the destination Elasticsearch domain.</p>
    pub fn destination_domain_info(mut self, input: crate::types::DomainInformation) -> Self {
        self.destination_domain_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the <code><code>DomainInformation</code></code> for the destination Elasticsearch domain.</p>
    pub fn set_destination_domain_info(
        mut self,
        input: ::std::option::Option<crate::types::DomainInformation>,
    ) -> Self {
        self.destination_domain_info = input;
        self
    }
    /// <p>Specifies the connection alias that will be used by the customer for this connection.</p>
    pub fn connection_alias(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.connection_alias = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the connection alias that will be used by the customer for this connection.</p>
    pub fn set_connection_alias(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.connection_alias = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateOutboundCrossClusterSearchConnectionInput`](crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::create_outbound_cross_cluster_search_connection::CreateOutboundCrossClusterSearchConnectionInput {
                source_domain_info: self.source_domain_info
                ,
                destination_domain_info: self.destination_domain_info
                ,
                connection_alias: self.connection_alias
                ,
            }
        )
    }
}
