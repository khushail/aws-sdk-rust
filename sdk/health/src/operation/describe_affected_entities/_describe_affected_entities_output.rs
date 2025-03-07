// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAffectedEntitiesOutput {
    /// <p>The entities that match the filter criteria.</p>
    #[doc(hidden)]
    pub entities: ::std::option::Option<::std::vec::Vec<crate::types::AffectedEntity>>,
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    #[doc(hidden)]
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeAffectedEntitiesOutput {
    /// <p>The entities that match the filter criteria.</p>
    pub fn entities(&self) -> ::std::option::Option<&[crate::types::AffectedEntity]> {
        self.entities.as_deref()
    }
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_http::request_id::RequestId for DescribeAffectedEntitiesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeAffectedEntitiesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAffectedEntitiesOutput`](crate::operation::describe_affected_entities::DescribeAffectedEntitiesOutput).
    pub fn builder(
    ) -> crate::operation::describe_affected_entities::builders::DescribeAffectedEntitiesOutputBuilder
    {
        crate::operation::describe_affected_entities::builders::DescribeAffectedEntitiesOutputBuilder::default()
    }
}

/// A builder for [`DescribeAffectedEntitiesOutput`](crate::operation::describe_affected_entities::DescribeAffectedEntitiesOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DescribeAffectedEntitiesOutputBuilder {
    pub(crate) entities: ::std::option::Option<::std::vec::Vec<crate::types::AffectedEntity>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeAffectedEntitiesOutputBuilder {
    /// Appends an item to `entities`.
    ///
    /// To override the contents of this collection use [`set_entities`](Self::set_entities).
    ///
    /// <p>The entities that match the filter criteria.</p>
    pub fn entities(mut self, input: crate::types::AffectedEntity) -> Self {
        let mut v = self.entities.unwrap_or_default();
        v.push(input);
        self.entities = ::std::option::Option::Some(v);
        self
    }
    /// <p>The entities that match the filter criteria.</p>
    pub fn set_entities(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AffectedEntity>>,
    ) -> Self {
        self.entities = input;
        self
    }
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>If the results of a search are large, only a portion of the results are returned, and a <code>nextToken</code> pagination token is returned in the response. To retrieve the next batch of results, reissue the search request and include the returned token. When all results have been returned, the response does not contain a pagination token value.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAffectedEntitiesOutput`](crate::operation::describe_affected_entities::DescribeAffectedEntitiesOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_affected_entities::DescribeAffectedEntitiesOutput {
        crate::operation::describe_affected_entities::DescribeAffectedEntitiesOutput {
            entities: self.entities,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
