// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Usage allocations allow you to split usage into buckets by tags.</p>
/// <p>Each <code>UsageAllocation</code> indicates the usage quantity for a specific set of tags.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UsageAllocation {
    /// <p>The total quantity allocated to this bucket of usage.</p>
    #[doc(hidden)]
    pub allocated_usage_quantity: ::std::option::Option<i32>,
    /// <p>The set of tags that define the bucket of usage. For the bucket of items with no tags, this parameter can be left out.</p>
    #[doc(hidden)]
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl UsageAllocation {
    /// <p>The total quantity allocated to this bucket of usage.</p>
    pub fn allocated_usage_quantity(&self) -> ::std::option::Option<i32> {
        self.allocated_usage_quantity
    }
    /// <p>The set of tags that define the bucket of usage. For the bucket of items with no tags, this parameter can be left out.</p>
    pub fn tags(&self) -> ::std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl UsageAllocation {
    /// Creates a new builder-style object to manufacture [`UsageAllocation`](crate::types::UsageAllocation).
    pub fn builder() -> crate::types::builders::UsageAllocationBuilder {
        crate::types::builders::UsageAllocationBuilder::default()
    }
}

/// A builder for [`UsageAllocation`](crate::types::UsageAllocation).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UsageAllocationBuilder {
    pub(crate) allocated_usage_quantity: ::std::option::Option<i32>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl UsageAllocationBuilder {
    /// <p>The total quantity allocated to this bucket of usage.</p>
    pub fn allocated_usage_quantity(mut self, input: i32) -> Self {
        self.allocated_usage_quantity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total quantity allocated to this bucket of usage.</p>
    pub fn set_allocated_usage_quantity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.allocated_usage_quantity = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The set of tags that define the bucket of usage. For the bucket of items with no tags, this parameter can be left out.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The set of tags that define the bucket of usage. For the bucket of items with no tags, this parameter can be left out.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`UsageAllocation`](crate::types::UsageAllocation).
    pub fn build(self) -> crate::types::UsageAllocation {
        crate::types::UsageAllocation {
            allocated_usage_quantity: self.allocated_usage_quantity,
            tags: self.tags,
        }
    }
}
