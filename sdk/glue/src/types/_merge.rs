// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies a transform that merges a <code>DynamicFrame</code> with a staging <code>DynamicFrame</code> based on the specified primary keys to identify records. Duplicate records (records with the same primary keys) are not de-duplicated. </p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Merge {
    /// <p>The name of the transform node.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The data inputs identified by their node names.</p>
    #[doc(hidden)]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The source <code>DynamicFrame</code> that will be merged with a staging <code>DynamicFrame</code>.</p>
    #[doc(hidden)]
    pub source: ::std::option::Option<::std::string::String>,
    /// <p>The list of primary key fields to match records from the source and staging dynamic frames.</p>
    #[doc(hidden)]
    pub primary_keys:
        ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
}
impl Merge {
    /// <p>The name of the transform node.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The data inputs identified by their node names.</p>
    pub fn inputs(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inputs.as_deref()
    }
    /// <p>The source <code>DynamicFrame</code> that will be merged with a staging <code>DynamicFrame</code>.</p>
    pub fn source(&self) -> ::std::option::Option<&str> {
        self.source.as_deref()
    }
    /// <p>The list of primary key fields to match records from the source and staging dynamic frames.</p>
    pub fn primary_keys(&self) -> ::std::option::Option<&[::std::vec::Vec<::std::string::String>]> {
        self.primary_keys.as_deref()
    }
}
impl Merge {
    /// Creates a new builder-style object to manufacture [`Merge`](crate::types::Merge).
    pub fn builder() -> crate::types::builders::MergeBuilder {
        crate::types::builders::MergeBuilder::default()
    }
}

/// A builder for [`Merge`](crate::types::Merge).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MergeBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) source: ::std::option::Option<::std::string::String>,
    pub(crate) primary_keys:
        ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
}
impl MergeBuilder {
    /// <p>The name of the transform node.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the transform node.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `inputs`.
    ///
    /// To override the contents of this collection use [`set_inputs`](Self::set_inputs).
    ///
    /// <p>The data inputs identified by their node names.</p>
    pub fn inputs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.inputs.unwrap_or_default();
        v.push(input.into());
        self.inputs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The data inputs identified by their node names.</p>
    pub fn set_inputs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inputs = input;
        self
    }
    /// <p>The source <code>DynamicFrame</code> that will be merged with a staging <code>DynamicFrame</code>.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source <code>DynamicFrame</code> that will be merged with a staging <code>DynamicFrame</code>.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// Appends an item to `primary_keys`.
    ///
    /// To override the contents of this collection use [`set_primary_keys`](Self::set_primary_keys).
    ///
    /// <p>The list of primary key fields to match records from the source and staging dynamic frames.</p>
    pub fn primary_keys(mut self, input: ::std::vec::Vec<::std::string::String>) -> Self {
        let mut v = self.primary_keys.unwrap_or_default();
        v.push(input);
        self.primary_keys = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of primary key fields to match records from the source and staging dynamic frames.</p>
    pub fn set_primary_keys(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
    ) -> Self {
        self.primary_keys = input;
        self
    }
    /// Consumes the builder and constructs a [`Merge`](crate::types::Merge).
    pub fn build(self) -> crate::types::Merge {
        crate::types::Merge {
            name: self.name,
            inputs: self.inputs,
            source: self.source,
            primary_keys: self.primary_keys,
        }
    }
}
