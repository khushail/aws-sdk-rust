// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportResourcesToDraftAppVersionInput {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    #[doc(hidden)]
    pub app_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Names (ARNs) for the resources.</p>
    #[doc(hidden)]
    pub source_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p> A list of terraform file s3 URLs you need to import. </p>
    #[doc(hidden)]
    pub terraform_sources: ::std::option::Option<::std::vec::Vec<crate::types::TerraformSource>>,
    /// <p>The import strategy you would like to set to import resources into Resilience Hub application.</p>
    #[doc(hidden)]
    pub import_strategy: ::std::option::Option<crate::types::ResourceImportStrategyType>,
    /// <p>The input sources of the Amazon Elastic Kubernetes Service resources you need to import.</p>
    #[doc(hidden)]
    pub eks_sources: ::std::option::Option<::std::vec::Vec<crate::types::EksSource>>,
}
impl ImportResourcesToDraftAppVersionInput {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(&self) -> ::std::option::Option<&str> {
        self.app_arn.as_deref()
    }
    /// <p>The Amazon Resource Names (ARNs) for the resources.</p>
    pub fn source_arns(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.source_arns.as_deref()
    }
    /// <p> A list of terraform file s3 URLs you need to import. </p>
    pub fn terraform_sources(&self) -> ::std::option::Option<&[crate::types::TerraformSource]> {
        self.terraform_sources.as_deref()
    }
    /// <p>The import strategy you would like to set to import resources into Resilience Hub application.</p>
    pub fn import_strategy(
        &self,
    ) -> ::std::option::Option<&crate::types::ResourceImportStrategyType> {
        self.import_strategy.as_ref()
    }
    /// <p>The input sources of the Amazon Elastic Kubernetes Service resources you need to import.</p>
    pub fn eks_sources(&self) -> ::std::option::Option<&[crate::types::EksSource]> {
        self.eks_sources.as_deref()
    }
}
impl ImportResourcesToDraftAppVersionInput {
    /// Creates a new builder-style object to manufacture [`ImportResourcesToDraftAppVersionInput`](crate::operation::import_resources_to_draft_app_version::ImportResourcesToDraftAppVersionInput).
    pub fn builder() -> crate::operation::import_resources_to_draft_app_version::builders::ImportResourcesToDraftAppVersionInputBuilder{
        crate::operation::import_resources_to_draft_app_version::builders::ImportResourcesToDraftAppVersionInputBuilder::default()
    }
}

/// A builder for [`ImportResourcesToDraftAppVersionInput`](crate::operation::import_resources_to_draft_app_version::ImportResourcesToDraftAppVersionInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportResourcesToDraftAppVersionInputBuilder {
    pub(crate) app_arn: ::std::option::Option<::std::string::String>,
    pub(crate) source_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) terraform_sources:
        ::std::option::Option<::std::vec::Vec<crate::types::TerraformSource>>,
    pub(crate) import_strategy: ::std::option::Option<crate::types::ResourceImportStrategyType>,
    pub(crate) eks_sources: ::std::option::Option<::std::vec::Vec<crate::types::EksSource>>,
}
impl ImportResourcesToDraftAppVersionInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn app_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.app_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Resilience Hub application. The format for this ARN is: arn:<code>partition</code>:resiliencehub:<code>region</code>:<code>account</code>:app/<code>app-id</code>. For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html"> Amazon Resource Names (ARNs)</a> in the <i>AWS General Reference</i> guide.</p>
    pub fn set_app_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.app_arn = input;
        self
    }
    /// Appends an item to `source_arns`.
    ///
    /// To override the contents of this collection use [`set_source_arns`](Self::set_source_arns).
    ///
    /// <p>The Amazon Resource Names (ARNs) for the resources.</p>
    pub fn source_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.source_arns.unwrap_or_default();
        v.push(input.into());
        self.source_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Resource Names (ARNs) for the resources.</p>
    pub fn set_source_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.source_arns = input;
        self
    }
    /// Appends an item to `terraform_sources`.
    ///
    /// To override the contents of this collection use [`set_terraform_sources`](Self::set_terraform_sources).
    ///
    /// <p> A list of terraform file s3 URLs you need to import. </p>
    pub fn terraform_sources(mut self, input: crate::types::TerraformSource) -> Self {
        let mut v = self.terraform_sources.unwrap_or_default();
        v.push(input);
        self.terraform_sources = ::std::option::Option::Some(v);
        self
    }
    /// <p> A list of terraform file s3 URLs you need to import. </p>
    pub fn set_terraform_sources(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TerraformSource>>,
    ) -> Self {
        self.terraform_sources = input;
        self
    }
    /// <p>The import strategy you would like to set to import resources into Resilience Hub application.</p>
    pub fn import_strategy(mut self, input: crate::types::ResourceImportStrategyType) -> Self {
        self.import_strategy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The import strategy you would like to set to import resources into Resilience Hub application.</p>
    pub fn set_import_strategy(
        mut self,
        input: ::std::option::Option<crate::types::ResourceImportStrategyType>,
    ) -> Self {
        self.import_strategy = input;
        self
    }
    /// Appends an item to `eks_sources`.
    ///
    /// To override the contents of this collection use [`set_eks_sources`](Self::set_eks_sources).
    ///
    /// <p>The input sources of the Amazon Elastic Kubernetes Service resources you need to import.</p>
    pub fn eks_sources(mut self, input: crate::types::EksSource) -> Self {
        let mut v = self.eks_sources.unwrap_or_default();
        v.push(input);
        self.eks_sources = ::std::option::Option::Some(v);
        self
    }
    /// <p>The input sources of the Amazon Elastic Kubernetes Service resources you need to import.</p>
    pub fn set_eks_sources(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EksSource>>,
    ) -> Self {
        self.eks_sources = input;
        self
    }
    /// Consumes the builder and constructs a [`ImportResourcesToDraftAppVersionInput`](crate::operation::import_resources_to_draft_app_version::ImportResourcesToDraftAppVersionInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::import_resources_to_draft_app_version::ImportResourcesToDraftAppVersionInput, ::aws_smithy_http::operation::error::BuildError>{
        ::std::result::Result::Ok(
            crate::operation::import_resources_to_draft_app_version::ImportResourcesToDraftAppVersionInput {
                app_arn: self.app_arn
                ,
                source_arns: self.source_arns
                ,
                terraform_sources: self.terraform_sources
                ,
                import_strategy: self.import_strategy
                ,
                eks_sources: self.eks_sources
                ,
            }
        )
    }
}
