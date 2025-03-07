// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The tags to apply to a resource when the resource is being created. When you specify a tag, you must specify the resource type to tag, otherwise the request will fail.</p> <note>
/// <p>The <code>Valid Values</code> lists all the resource types that can be tagged. However, the action you're using might not support tagging all of these resource types. If you try to tag a resource type that is unsupported for the action you're using, you'll get an error.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagSpecification {
    /// <p>The type of resource to tag on creation.</p>
    #[doc(hidden)]
    pub resource_type: ::std::option::Option<crate::types::ResourceType>,
    /// <p>The tags to apply to the resource.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TagSpecification {
    /// <p>The type of resource to tag on creation.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&crate::types::ResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>The tags to apply to the resource.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl TagSpecification {
    /// Creates a new builder-style object to manufacture [`TagSpecification`](crate::types::TagSpecification).
    pub fn builder() -> crate::types::builders::TagSpecificationBuilder {
        crate::types::builders::TagSpecificationBuilder::default()
    }
}

/// A builder for [`TagSpecification`](crate::types::TagSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TagSpecificationBuilder {
    pub(crate) resource_type: ::std::option::Option<crate::types::ResourceType>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TagSpecificationBuilder {
    /// <p>The type of resource to tag on creation.</p>
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of resource to tag on creation.</p>
    pub fn set_resource_type(
        mut self,
        input: ::std::option::Option<crate::types::ResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to apply to the resource.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the resource.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`TagSpecification`](crate::types::TagSpecification).
    pub fn build(self) -> crate::types::TagSpecification {
        crate::types::TagSpecification {
            resource_type: self.resource_type,
            tags: self.tags,
        }
    }
}
