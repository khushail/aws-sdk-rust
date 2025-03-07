// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies a transform that uses custom code you provide to perform the data transformation. The output is a collection of DynamicFrames.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CustomCode {
    /// <p>The name of the transform node.</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The data inputs identified by their node names.</p>
    #[doc(hidden)]
    pub inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The custom code that is used to perform the data transformation.</p>
    #[doc(hidden)]
    pub code: ::std::option::Option<::std::string::String>,
    /// <p>The name defined for the custom code node class.</p>
    #[doc(hidden)]
    pub class_name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the data schema for the custom code transform.</p>
    #[doc(hidden)]
    pub output_schemas: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>,
}
impl CustomCode {
    /// <p>The name of the transform node.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The data inputs identified by their node names.</p>
    pub fn inputs(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.inputs.as_deref()
    }
    /// <p>The custom code that is used to perform the data transformation.</p>
    pub fn code(&self) -> ::std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>The name defined for the custom code node class.</p>
    pub fn class_name(&self) -> ::std::option::Option<&str> {
        self.class_name.as_deref()
    }
    /// <p>Specifies the data schema for the custom code transform.</p>
    pub fn output_schemas(&self) -> ::std::option::Option<&[crate::types::GlueSchema]> {
        self.output_schemas.as_deref()
    }
}
impl CustomCode {
    /// Creates a new builder-style object to manufacture [`CustomCode`](crate::types::CustomCode).
    pub fn builder() -> crate::types::builders::CustomCodeBuilder {
        crate::types::builders::CustomCodeBuilder::default()
    }
}

/// A builder for [`CustomCode`](crate::types::CustomCode).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CustomCodeBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) inputs: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) code: ::std::option::Option<::std::string::String>,
    pub(crate) class_name: ::std::option::Option<::std::string::String>,
    pub(crate) output_schemas: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>,
}
impl CustomCodeBuilder {
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
    /// <p>The custom code that is used to perform the data transformation.</p>
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The custom code that is used to perform the data transformation.</p>
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>The name defined for the custom code node class.</p>
    pub fn class_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.class_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name defined for the custom code node class.</p>
    pub fn set_class_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.class_name = input;
        self
    }
    /// Appends an item to `output_schemas`.
    ///
    /// To override the contents of this collection use [`set_output_schemas`](Self::set_output_schemas).
    ///
    /// <p>Specifies the data schema for the custom code transform.</p>
    pub fn output_schemas(mut self, input: crate::types::GlueSchema) -> Self {
        let mut v = self.output_schemas.unwrap_or_default();
        v.push(input);
        self.output_schemas = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the data schema for the custom code transform.</p>
    pub fn set_output_schemas(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::GlueSchema>>,
    ) -> Self {
        self.output_schemas = input;
        self
    }
    /// Consumes the builder and constructs a [`CustomCode`](crate::types::CustomCode).
    pub fn build(self) -> crate::types::CustomCode {
        crate::types::CustomCode {
            name: self.name,
            inputs: self.inputs,
            code: self.code,
            class_name: self.class_name,
            output_schemas: self.output_schemas,
        }
    }
}
