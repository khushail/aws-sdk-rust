// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes updates to the execution property groups for a Flink-based Kinesis Data Analytics application or a Studio notebook.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnvironmentPropertyUpdates {
    /// <p>Describes updates to the execution property groups.</p>
    #[doc(hidden)]
    pub property_groups: ::std::option::Option<::std::vec::Vec<crate::types::PropertyGroup>>,
}
impl EnvironmentPropertyUpdates {
    /// <p>Describes updates to the execution property groups.</p>
    pub fn property_groups(&self) -> ::std::option::Option<&[crate::types::PropertyGroup]> {
        self.property_groups.as_deref()
    }
}
impl EnvironmentPropertyUpdates {
    /// Creates a new builder-style object to manufacture [`EnvironmentPropertyUpdates`](crate::types::EnvironmentPropertyUpdates).
    pub fn builder() -> crate::types::builders::EnvironmentPropertyUpdatesBuilder {
        crate::types::builders::EnvironmentPropertyUpdatesBuilder::default()
    }
}

/// A builder for [`EnvironmentPropertyUpdates`](crate::types::EnvironmentPropertyUpdates).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EnvironmentPropertyUpdatesBuilder {
    pub(crate) property_groups: ::std::option::Option<::std::vec::Vec<crate::types::PropertyGroup>>,
}
impl EnvironmentPropertyUpdatesBuilder {
    /// Appends an item to `property_groups`.
    ///
    /// To override the contents of this collection use [`set_property_groups`](Self::set_property_groups).
    ///
    /// <p>Describes updates to the execution property groups.</p>
    pub fn property_groups(mut self, input: crate::types::PropertyGroup) -> Self {
        let mut v = self.property_groups.unwrap_or_default();
        v.push(input);
        self.property_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>Describes updates to the execution property groups.</p>
    pub fn set_property_groups(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::PropertyGroup>>,
    ) -> Self {
        self.property_groups = input;
        self
    }
    /// Consumes the builder and constructs a [`EnvironmentPropertyUpdates`](crate::types::EnvironmentPropertyUpdates).
    pub fn build(self) -> crate::types::EnvironmentPropertyUpdates {
        crate::types::EnvironmentPropertyUpdates {
            property_groups: self.property_groups,
        }
    }
}
