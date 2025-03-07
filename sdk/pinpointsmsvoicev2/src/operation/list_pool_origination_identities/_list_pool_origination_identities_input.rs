// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListPoolOriginationIdentitiesInput {
    /// <p>The unique identifier for the pool. This value can be either the PoolId or PoolArn.</p>
    #[doc(hidden)]
    pub pool_id: ::std::option::Option<::std::string::String>,
    /// <p>An array of PoolOriginationIdentitiesFilter objects to filter the results..</p>
    #[doc(hidden)]
    pub filters:
        ::std::option::Option<::std::vec::Vec<crate::types::PoolOriginationIdentitiesFilter>>,
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return per each request.</p>
    #[doc(hidden)]
    pub max_results: ::std::option::Option<i32>,
}
impl ListPoolOriginationIdentitiesInput {
    /// <p>The unique identifier for the pool. This value can be either the PoolId or PoolArn.</p>
    pub fn pool_id(&self) -> ::std::option::Option<&str> {
        self.pool_id.as_deref()
    }
    /// <p>An array of PoolOriginationIdentitiesFilter objects to filter the results..</p>
    pub fn filters(
        &self,
    ) -> ::std::option::Option<&[crate::types::PoolOriginationIdentitiesFilter]> {
        self.filters.as_deref()
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl ListPoolOriginationIdentitiesInput {
    /// Creates a new builder-style object to manufacture [`ListPoolOriginationIdentitiesInput`](crate::operation::list_pool_origination_identities::ListPoolOriginationIdentitiesInput).
    pub fn builder() -> crate::operation::list_pool_origination_identities::builders::ListPoolOriginationIdentitiesInputBuilder{
        crate::operation::list_pool_origination_identities::builders::ListPoolOriginationIdentitiesInputBuilder::default()
    }
}

/// A builder for [`ListPoolOriginationIdentitiesInput`](crate::operation::list_pool_origination_identities::ListPoolOriginationIdentitiesInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ListPoolOriginationIdentitiesInputBuilder {
    pub(crate) pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) filters:
        ::std::option::Option<::std::vec::Vec<crate::types::PoolOriginationIdentitiesFilter>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl ListPoolOriginationIdentitiesInputBuilder {
    /// <p>The unique identifier for the pool. This value can be either the PoolId or PoolArn.</p>
    pub fn pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the pool. This value can be either the PoolId or PoolArn.</p>
    pub fn set_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pool_id = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An array of PoolOriginationIdentitiesFilter objects to filter the results..</p>
    pub fn filters(mut self, input: crate::types::PoolOriginationIdentitiesFilter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of PoolOriginationIdentitiesFilter objects to filter the results..</p>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::PoolOriginationIdentitiesFilter>,
        >,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// Consumes the builder and constructs a [`ListPoolOriginationIdentitiesInput`](crate::operation::list_pool_origination_identities::ListPoolOriginationIdentitiesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_pool_origination_identities::ListPoolOriginationIdentitiesInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_pool_origination_identities::ListPoolOriginationIdentitiesInput {
                pool_id: self.pool_id
                ,
                filters: self.filters
                ,
                next_token: self.next_token
                ,
                max_results: self.max_results
                ,
            }
        )
    }
}
