// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListVirtualInterfaceTestHistoryInput {
    /// <p>The ID of the virtual interface failover test.</p>
    #[doc(hidden)]
    pub test_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the virtual interface that was tested.</p>
    #[doc(hidden)]
    pub virtual_interface_id: ::std::option::Option<::std::string::String>,
    /// <p>The BGP peers that were placed in the DOWN state during the virtual interface failover test.</p>
    #[doc(hidden)]
    pub bgp_peers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The status of the virtual interface failover test.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    /// <p>If <code>MaxResults</code> is given a value larger than 100, only 100 results are returned.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl ListVirtualInterfaceTestHistoryInput {
    /// <p>The ID of the virtual interface failover test.</p>
    pub fn test_id(&self) -> ::std::option::Option<&str> {
        self.test_id.as_deref()
    }
    /// <p>The ID of the virtual interface that was tested.</p>
    pub fn virtual_interface_id(&self) -> ::std::option::Option<&str> {
        self.virtual_interface_id.as_deref()
    }
    /// <p>The BGP peers that were placed in the DOWN state during the virtual interface failover test.</p>
    pub fn bgp_peers(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.bgp_peers.as_deref()
    }
    /// <p>The status of the virtual interface failover test.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    /// <p>If <code>MaxResults</code> is given a value larger than 100, only 100 results are returned.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ListVirtualInterfaceTestHistoryInput {
    /// Creates a new builder-style object to manufacture [`ListVirtualInterfaceTestHistoryInput`](crate::operation::list_virtual_interface_test_history::ListVirtualInterfaceTestHistoryInput).
    pub fn builder() -> crate::operation::list_virtual_interface_test_history::builders::ListVirtualInterfaceTestHistoryInputBuilder{
        crate::operation::list_virtual_interface_test_history::builders::ListVirtualInterfaceTestHistoryInputBuilder::default()
    }
}

/// A builder for [`ListVirtualInterfaceTestHistoryInput`](crate::operation::list_virtual_interface_test_history::ListVirtualInterfaceTestHistoryInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListVirtualInterfaceTestHistoryInputBuilder {
    pub(crate) test_id: ::std::option::Option<::std::string::String>,
    pub(crate) virtual_interface_id: ::std::option::Option<::std::string::String>,
    pub(crate) bgp_peers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl ListVirtualInterfaceTestHistoryInputBuilder {
    /// <p>The ID of the virtual interface failover test.</p>
    pub fn test_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.test_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the virtual interface failover test.</p>
    pub fn set_test_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.test_id = input;
        self
    }
    /// <p>The ID of the virtual interface that was tested.</p>
    pub fn virtual_interface_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.virtual_interface_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the virtual interface that was tested.</p>
    pub fn set_virtual_interface_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.virtual_interface_id = input;
        self
    }
    /// Appends an item to `bgp_peers`.
    ///
    /// To override the contents of this collection use [`set_bgp_peers`](Self::set_bgp_peers).
    ///
    /// <p>The BGP peers that were placed in the DOWN state during the virtual interface failover test.</p>
    pub fn bgp_peers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.bgp_peers.unwrap_or_default();
        v.push(input.into());
        self.bgp_peers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The BGP peers that were placed in the DOWN state during the virtual interface failover test.</p>
    pub fn set_bgp_peers(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.bgp_peers = input;
        self
    }
    /// <p>The status of the virtual interface failover test.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status of the virtual interface failover test.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    /// <p>If <code>MaxResults</code> is given a value larger than 100, only 100 results are returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    /// <p>If <code>MaxResults</code> is given a value larger than 100, only 100 results are returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Consumes the builder and constructs a [`ListVirtualInterfaceTestHistoryInput`](crate::operation::list_virtual_interface_test_history::ListVirtualInterfaceTestHistoryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_virtual_interface_test_history::ListVirtualInterfaceTestHistoryInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_virtual_interface_test_history::ListVirtualInterfaceTestHistoryInput {
                test_id: self.test_id
                ,
                virtual_interface_id: self.virtual_interface_id
                ,
                bgp_peers: self.bgp_peers
                ,
                status: self.status
                ,
                max_results: self.max_results
                ,
                next_token: self.next_token
                ,
            }
        )
    }
}
