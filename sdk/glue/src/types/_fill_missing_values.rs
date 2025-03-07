// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies a transform that locates records in the dataset that have missing values and adds a new field with a value determined by imputation. The input data set is used to train the machine learning model that determines what the missing value should be.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FillMissingValues {
    /// <p>The name of the transform node.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The data inputs identified by their node names.</p>
    #[doc(hidden)]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A JSON path to a variable in the data structure for the dataset that is imputed.</p>
    #[doc(hidden)]
    pub imputed_path: ::std::option::Option<::std::string::String>,
    /// <p>A JSON path to a variable in the data structure for the dataset that is filled.</p>
    #[doc(hidden)]
    pub filled_path: ::std::option::Option<::std::string::String>,
}
impl FillMissingValues {
    /// <p>The name of the transform node.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The data inputs identified by their node names.</p>
    pub fn inputs(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inputs.as_deref()
    }
    /// <p>A JSON path to a variable in the data structure for the dataset that is imputed.</p>
    pub fn imputed_path(&self) -> ::std::option::Option<&str> {
        self.imputed_path.as_deref()
    }
    /// <p>A JSON path to a variable in the data structure for the dataset that is filled.</p>
    pub fn filled_path(&self) -> ::std::option::Option<&str> {
        self.filled_path.as_deref()
    }
}
impl FillMissingValues {
    /// Creates a new builder-style object to manufacture [`FillMissingValues`](crate::types::FillMissingValues).
    pub fn builder() -> crate::types::builders::FillMissingValuesBuilder {
        crate::types::builders::FillMissingValuesBuilder::default()
    }
}

/// A builder for [`FillMissingValues`](crate::types::FillMissingValues).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct FillMissingValuesBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) imputed_path: ::std::option::Option<::std::string::String>,
    pub(crate) filled_path: ::std::option::Option<::std::string::String>,
}
impl FillMissingValuesBuilder {
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
    /// <p>A JSON path to a variable in the data structure for the dataset that is imputed.</p>
    pub fn imputed_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.imputed_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A JSON path to a variable in the data structure for the dataset that is imputed.</p>
    pub fn set_imputed_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.imputed_path = input;
        self
    }
    /// <p>A JSON path to a variable in the data structure for the dataset that is filled.</p>
    pub fn filled_path(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.filled_path = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A JSON path to a variable in the data structure for the dataset that is filled.</p>
    pub fn set_filled_path(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.filled_path = input;
        self
    }
    /// Consumes the builder and constructs a [`FillMissingValues`](crate::types::FillMissingValues).
    pub fn build(self) -> crate::types::FillMissingValues {
        crate::types::FillMissingValues {
            name: self.name,
            inputs: self.inputs,
            imputed_path: self.imputed_path,
            filled_path: self.filled_path,
        }
    }
}
