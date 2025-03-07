// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Container image configuration object for the monitoring job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModelQualityAppSpecification {
    /// <p>The address of the container image that the monitoring job runs.</p>
    #[doc(hidden)]
    pub image_uri: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the entrypoint for a container that the monitoring job runs.</p>
    #[doc(hidden)]
    pub container_entrypoint: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>An array of arguments for the container used to run the monitoring job.</p>
    #[doc(hidden)]
    pub container_arguments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>An Amazon S3 URI to a script that is called per row prior to running analysis. It can base64 decode the payload and convert it into a flatted json so that the built-in container can use the converted data. Applicable only for the built-in (first party) containers.</p>
    #[doc(hidden)]
    pub record_preprocessor_source_uri: ::std::option::Option<::std::string::String>,
    /// <p>An Amazon S3 URI to a script that is called after analysis has been performed. Applicable only for the built-in (first party) containers.</p>
    #[doc(hidden)]
    pub post_analytics_processor_source_uri: ::std::option::Option<::std::string::String>,
    /// <p>The machine learning problem type of the model that the monitoring job monitors.</p>
    #[doc(hidden)]
    pub problem_type: ::std::option::Option<crate::types::MonitoringProblemType>,
    /// <p>Sets the environment variables in the container that the monitoring job runs.</p>
    #[doc(hidden)]
    pub environment: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl ModelQualityAppSpecification {
    /// <p>The address of the container image that the monitoring job runs.</p>
    pub fn image_uri(&self) -> ::std::option::Option<&str> {
        self.image_uri.as_deref()
    }
    /// <p>Specifies the entrypoint for a container that the monitoring job runs.</p>
    pub fn container_entrypoint(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.container_entrypoint.as_deref()
    }
    /// <p>An array of arguments for the container used to run the monitoring job.</p>
    pub fn container_arguments(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.container_arguments.as_deref()
    }
    /// <p>An Amazon S3 URI to a script that is called per row prior to running analysis. It can base64 decode the payload and convert it into a flatted json so that the built-in container can use the converted data. Applicable only for the built-in (first party) containers.</p>
    pub fn record_preprocessor_source_uri(&self) -> ::std::option::Option<&str> {
        self.record_preprocessor_source_uri.as_deref()
    }
    /// <p>An Amazon S3 URI to a script that is called after analysis has been performed. Applicable only for the built-in (first party) containers.</p>
    pub fn post_analytics_processor_source_uri(&self) -> ::std::option::Option<&str> {
        self.post_analytics_processor_source_uri.as_deref()
    }
    /// <p>The machine learning problem type of the model that the monitoring job monitors.</p>
    pub fn problem_type(&self) -> ::std::option::Option<&crate::types::MonitoringProblemType> {
        self.problem_type.as_ref()
    }
    /// <p>Sets the environment variables in the container that the monitoring job runs.</p>
    pub fn environment(
        &self,
    ) -> ::std::option::Option<
        &::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.environment.as_ref()
    }
}
impl ModelQualityAppSpecification {
    /// Creates a new builder-style object to manufacture [`ModelQualityAppSpecification`](crate::types::ModelQualityAppSpecification).
    pub fn builder() -> crate::types::builders::ModelQualityAppSpecificationBuilder {
        crate::types::builders::ModelQualityAppSpecificationBuilder::default()
    }
}

/// A builder for [`ModelQualityAppSpecification`](crate::types::ModelQualityAppSpecification).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ModelQualityAppSpecificationBuilder {
    pub(crate) image_uri: ::std::option::Option<::std::string::String>,
    pub(crate) container_entrypoint: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) container_arguments: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) record_preprocessor_source_uri: ::std::option::Option<::std::string::String>,
    pub(crate) post_analytics_processor_source_uri: ::std::option::Option<::std::string::String>,
    pub(crate) problem_type: ::std::option::Option<crate::types::MonitoringProblemType>,
    pub(crate) environment: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
}
impl ModelQualityAppSpecificationBuilder {
    /// <p>The address of the container image that the monitoring job runs.</p>
    pub fn image_uri(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The address of the container image that the monitoring job runs.</p>
    pub fn set_image_uri(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_uri = input;
        self
    }
    /// Appends an item to `container_entrypoint`.
    ///
    /// To override the contents of this collection use [`set_container_entrypoint`](Self::set_container_entrypoint).
    ///
    /// <p>Specifies the entrypoint for a container that the monitoring job runs.</p>
    pub fn container_entrypoint(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.container_entrypoint.unwrap_or_default();
        v.push(input.into());
        self.container_entrypoint = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the entrypoint for a container that the monitoring job runs.</p>
    pub fn set_container_entrypoint(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.container_entrypoint = input;
        self
    }
    /// Appends an item to `container_arguments`.
    ///
    /// To override the contents of this collection use [`set_container_arguments`](Self::set_container_arguments).
    ///
    /// <p>An array of arguments for the container used to run the monitoring job.</p>
    pub fn container_arguments(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.container_arguments.unwrap_or_default();
        v.push(input.into());
        self.container_arguments = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array of arguments for the container used to run the monitoring job.</p>
    pub fn set_container_arguments(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.container_arguments = input;
        self
    }
    /// <p>An Amazon S3 URI to a script that is called per row prior to running analysis. It can base64 decode the payload and convert it into a flatted json so that the built-in container can use the converted data. Applicable only for the built-in (first party) containers.</p>
    pub fn record_preprocessor_source_uri(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.record_preprocessor_source_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An Amazon S3 URI to a script that is called per row prior to running analysis. It can base64 decode the payload and convert it into a flatted json so that the built-in container can use the converted data. Applicable only for the built-in (first party) containers.</p>
    pub fn set_record_preprocessor_source_uri(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.record_preprocessor_source_uri = input;
        self
    }
    /// <p>An Amazon S3 URI to a script that is called after analysis has been performed. Applicable only for the built-in (first party) containers.</p>
    pub fn post_analytics_processor_source_uri(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.post_analytics_processor_source_uri = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An Amazon S3 URI to a script that is called after analysis has been performed. Applicable only for the built-in (first party) containers.</p>
    pub fn set_post_analytics_processor_source_uri(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.post_analytics_processor_source_uri = input;
        self
    }
    /// <p>The machine learning problem type of the model that the monitoring job monitors.</p>
    pub fn problem_type(mut self, input: crate::types::MonitoringProblemType) -> Self {
        self.problem_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The machine learning problem type of the model that the monitoring job monitors.</p>
    pub fn set_problem_type(
        mut self,
        input: ::std::option::Option<crate::types::MonitoringProblemType>,
    ) -> Self {
        self.problem_type = input;
        self
    }
    /// Adds a key-value pair to `environment`.
    ///
    /// To override the contents of this collection use [`set_environment`](Self::set_environment).
    ///
    /// <p>Sets the environment variables in the container that the monitoring job runs.</p>
    pub fn environment(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.environment.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.environment = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Sets the environment variables in the container that the monitoring job runs.</p>
    pub fn set_environment(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.environment = input;
        self
    }
    /// Consumes the builder and constructs a [`ModelQualityAppSpecification`](crate::types::ModelQualityAppSpecification).
    pub fn build(self) -> crate::types::ModelQualityAppSpecification {
        crate::types::ModelQualityAppSpecification {
            image_uri: self.image_uri,
            container_entrypoint: self.container_entrypoint,
            container_arguments: self.container_arguments,
            record_preprocessor_source_uri: self.record_preprocessor_source_uri,
            post_analytics_processor_source_uri: self.post_analytics_processor_source_uri,
            problem_type: self.problem_type,
            environment: self.environment,
        }
    }
}
