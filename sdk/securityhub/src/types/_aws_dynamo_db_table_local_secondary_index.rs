// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a local secondary index for a DynamoDB table.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsDynamoDbTableLocalSecondaryIndex {
    /// <p>The ARN of the index.</p>
    #[doc(hidden)]
    pub index_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the index.</p>
    #[doc(hidden)]
    pub index_name: ::std::option::Option<::std::string::String>,
    /// <p>The complete key schema for the index.</p>
    #[doc(hidden)]
    pub key_schema: ::std::option::Option<::std::vec::Vec<crate::types::AwsDynamoDbTableKeySchema>>,
    /// <p>Attributes that are copied from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    #[doc(hidden)]
    pub projection: ::std::option::Option<crate::types::AwsDynamoDbTableProjection>,
}
impl AwsDynamoDbTableLocalSecondaryIndex {
    /// <p>The ARN of the index.</p>
    pub fn index_arn(&self) -> ::std::option::Option<&str> {
        self.index_arn.as_deref()
    }
    /// <p>The name of the index.</p>
    pub fn index_name(&self) -> ::std::option::Option<&str> {
        self.index_name.as_deref()
    }
    /// <p>The complete key schema for the index.</p>
    pub fn key_schema(&self) -> ::std::option::Option<&[crate::types::AwsDynamoDbTableKeySchema]> {
        self.key_schema.as_deref()
    }
    /// <p>Attributes that are copied from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    pub fn projection(&self) -> ::std::option::Option<&crate::types::AwsDynamoDbTableProjection> {
        self.projection.as_ref()
    }
}
impl AwsDynamoDbTableLocalSecondaryIndex {
    /// Creates a new builder-style object to manufacture [`AwsDynamoDbTableLocalSecondaryIndex`](crate::types::AwsDynamoDbTableLocalSecondaryIndex).
    pub fn builder() -> crate::types::builders::AwsDynamoDbTableLocalSecondaryIndexBuilder {
        crate::types::builders::AwsDynamoDbTableLocalSecondaryIndexBuilder::default()
    }
}

/// A builder for [`AwsDynamoDbTableLocalSecondaryIndex`](crate::types::AwsDynamoDbTableLocalSecondaryIndex).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsDynamoDbTableLocalSecondaryIndexBuilder {
    pub(crate) index_arn: ::std::option::Option<::std::string::String>,
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
    pub(crate) key_schema:
        ::std::option::Option<::std::vec::Vec<crate::types::AwsDynamoDbTableKeySchema>>,
    pub(crate) projection: ::std::option::Option<crate::types::AwsDynamoDbTableProjection>,
}
impl AwsDynamoDbTableLocalSecondaryIndexBuilder {
    /// <p>The ARN of the index.</p>
    pub fn index_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the index.</p>
    pub fn set_index_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_arn = input;
        self
    }
    /// <p>The name of the index.</p>
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the index.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// Appends an item to `key_schema`.
    ///
    /// To override the contents of this collection use [`set_key_schema`](Self::set_key_schema).
    ///
    /// <p>The complete key schema for the index.</p>
    pub fn key_schema(mut self, input: crate::types::AwsDynamoDbTableKeySchema) -> Self {
        let mut v = self.key_schema.unwrap_or_default();
        v.push(input);
        self.key_schema = ::std::option::Option::Some(v);
        self
    }
    /// <p>The complete key schema for the index.</p>
    pub fn set_key_schema(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AwsDynamoDbTableKeySchema>>,
    ) -> Self {
        self.key_schema = input;
        self
    }
    /// <p>Attributes that are copied from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    pub fn projection(mut self, input: crate::types::AwsDynamoDbTableProjection) -> Self {
        self.projection = ::std::option::Option::Some(input);
        self
    }
    /// <p>Attributes that are copied from the table into the index. These are in addition to the primary key attributes and index key attributes, which are automatically projected.</p>
    pub fn set_projection(
        mut self,
        input: ::std::option::Option<crate::types::AwsDynamoDbTableProjection>,
    ) -> Self {
        self.projection = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsDynamoDbTableLocalSecondaryIndex`](crate::types::AwsDynamoDbTableLocalSecondaryIndex).
    pub fn build(self) -> crate::types::AwsDynamoDbTableLocalSecondaryIndex {
        crate::types::AwsDynamoDbTableLocalSecondaryIndex {
            index_arn: self.index_arn,
            index_name: self.index_name,
            key_schema: self.key_schema,
            projection: self.projection,
        }
    }
}
