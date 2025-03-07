// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The input structure for the InputConfig in a VectorEnrichmentJob.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VectorEnrichmentJobInputConfig {
    /// <p>The input structure that defines the data source file type.</p>
    #[doc(hidden)]
    pub document_type: ::std::option::Option<crate::types::VectorEnrichmentJobDocumentType>,
    /// <p>The input structure for the data source that represents the storage type of the input data objects.</p>
    #[doc(hidden)]
    pub data_source_config:
        ::std::option::Option<crate::types::VectorEnrichmentJobDataSourceConfigInput>,
}
impl VectorEnrichmentJobInputConfig {
    /// <p>The input structure that defines the data source file type.</p>
    pub fn document_type(
        &self,
    ) -> ::std::option::Option<&crate::types::VectorEnrichmentJobDocumentType> {
        self.document_type.as_ref()
    }
    /// <p>The input structure for the data source that represents the storage type of the input data objects.</p>
    pub fn data_source_config(
        &self,
    ) -> ::std::option::Option<&crate::types::VectorEnrichmentJobDataSourceConfigInput> {
        self.data_source_config.as_ref()
    }
}
impl VectorEnrichmentJobInputConfig {
    /// Creates a new builder-style object to manufacture [`VectorEnrichmentJobInputConfig`](crate::types::VectorEnrichmentJobInputConfig).
    pub fn builder() -> crate::types::builders::VectorEnrichmentJobInputConfigBuilder {
        crate::types::builders::VectorEnrichmentJobInputConfigBuilder::default()
    }
}

/// A builder for [`VectorEnrichmentJobInputConfig`](crate::types::VectorEnrichmentJobInputConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct VectorEnrichmentJobInputConfigBuilder {
    pub(crate) document_type: ::std::option::Option<crate::types::VectorEnrichmentJobDocumentType>,
    pub(crate) data_source_config:
        ::std::option::Option<crate::types::VectorEnrichmentJobDataSourceConfigInput>,
}
impl VectorEnrichmentJobInputConfigBuilder {
    /// <p>The input structure that defines the data source file type.</p>
    pub fn document_type(mut self, input: crate::types::VectorEnrichmentJobDocumentType) -> Self {
        self.document_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The input structure that defines the data source file type.</p>
    pub fn set_document_type(
        mut self,
        input: ::std::option::Option<crate::types::VectorEnrichmentJobDocumentType>,
    ) -> Self {
        self.document_type = input;
        self
    }
    /// <p>The input structure for the data source that represents the storage type of the input data objects.</p>
    pub fn data_source_config(
        mut self,
        input: crate::types::VectorEnrichmentJobDataSourceConfigInput,
    ) -> Self {
        self.data_source_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The input structure for the data source that represents the storage type of the input data objects.</p>
    pub fn set_data_source_config(
        mut self,
        input: ::std::option::Option<crate::types::VectorEnrichmentJobDataSourceConfigInput>,
    ) -> Self {
        self.data_source_config = input;
        self
    }
    /// Consumes the builder and constructs a [`VectorEnrichmentJobInputConfig`](crate::types::VectorEnrichmentJobInputConfig).
    pub fn build(self) -> crate::types::VectorEnrichmentJobInputConfig {
        crate::types::VectorEnrichmentJobInputConfig {
            document_type: self.document_type,
            data_source_config: self.data_source_config,
        }
    }
}
