// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies an Amazon Redshift target.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AmazonRedshiftTarget {
    /// <p>The name of the Amazon Redshift target.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the data of the Amazon Reshift target node.</p>
    #[doc(hidden)]
    pub data: ::std::option::Option<crate::types::AmazonRedshiftNodeData>,
    /// <p>The nodes that are inputs to the data target.</p>
    #[doc(hidden)]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AmazonRedshiftTarget {
    /// <p>The name of the Amazon Redshift target.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Specifies the data of the Amazon Reshift target node.</p>
    pub fn data(&self) -> ::std::option::Option<&crate::types::AmazonRedshiftNodeData> {
        self.data.as_ref()
    }
    /// <p>The nodes that are inputs to the data target.</p>
    pub fn inputs(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inputs.as_deref()
    }
}
impl AmazonRedshiftTarget {
    /// Creates a new builder-style object to manufacture [`AmazonRedshiftTarget`](crate::types::AmazonRedshiftTarget).
    pub fn builder() -> crate::types::builders::AmazonRedshiftTargetBuilder {
        crate::types::builders::AmazonRedshiftTargetBuilder::default()
    }
}

/// A builder for [`AmazonRedshiftTarget`](crate::types::AmazonRedshiftTarget).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AmazonRedshiftTargetBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) data: ::std::option::Option<crate::types::AmazonRedshiftNodeData>,
    pub(crate) inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AmazonRedshiftTargetBuilder {
    /// <p>The name of the Amazon Redshift target.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon Redshift target.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Specifies the data of the Amazon Reshift target node.</p>
    pub fn data(mut self, input: crate::types::AmazonRedshiftNodeData) -> Self {
        self.data = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the data of the Amazon Reshift target node.</p>
    pub fn set_data(
        mut self,
        input: ::std::option::Option<crate::types::AmazonRedshiftNodeData>,
    ) -> Self {
        self.data = input;
        self
    }
    /// Appends an item to `inputs`.
    ///
    /// To override the contents of this collection use [`set_inputs`](Self::set_inputs).
    ///
    /// <p>The nodes that are inputs to the data target.</p>
    pub fn inputs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.inputs.unwrap_or_default();
        v.push(input.into());
        self.inputs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The nodes that are inputs to the data target.</p>
    pub fn set_inputs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inputs = input;
        self
    }
    /// Consumes the builder and constructs a [`AmazonRedshiftTarget`](crate::types::AmazonRedshiftTarget).
    pub fn build(self) -> crate::types::AmazonRedshiftTarget {
        crate::types::AmazonRedshiftTarget {
            name: self.name,
            data: self.data,
            inputs: self.inputs,
        }
    }
}
