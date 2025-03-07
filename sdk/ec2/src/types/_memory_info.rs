// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the memory for the instance type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MemoryInfo {
    /// <p>The size of the memory, in MiB.</p>
    #[doc(hidden)]
    pub size_in_mi_b: ::std::option::Option<i64>,
}
impl MemoryInfo {
    /// <p>The size of the memory, in MiB.</p>
    pub fn size_in_mi_b(&self) -> ::std::option::Option<i64> {
        self.size_in_mi_b
    }
}
impl MemoryInfo {
    /// Creates a new builder-style object to manufacture [`MemoryInfo`](crate::types::MemoryInfo).
    pub fn builder() -> crate::types::builders::MemoryInfoBuilder {
        crate::types::builders::MemoryInfoBuilder::default()
    }
}

/// A builder for [`MemoryInfo`](crate::types::MemoryInfo).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MemoryInfoBuilder {
    pub(crate) size_in_mi_b: ::std::option::Option<i64>,
}
impl MemoryInfoBuilder {
    /// <p>The size of the memory, in MiB.</p>
    pub fn size_in_mi_b(mut self, input: i64) -> Self {
        self.size_in_mi_b = ::std::option::Option::Some(input);
        self
    }
    /// <p>The size of the memory, in MiB.</p>
    pub fn set_size_in_mi_b(mut self, input: ::std::option::Option<i64>) -> Self {
        self.size_in_mi_b = input;
        self
    }
    /// Consumes the builder and constructs a [`MemoryInfo`](crate::types::MemoryInfo).
    pub fn build(self) -> crate::types::MemoryInfo {
        crate::types::MemoryInfo {
            size_in_mi_b: self.size_in_mi_b,
        }
    }
}
