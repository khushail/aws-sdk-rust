// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_role_from_db_cluster::_remove_role_from_db_cluster_output::RemoveRoleFromDbClusterOutputBuilder;

pub use crate::operation::remove_role_from_db_cluster::_remove_role_from_db_cluster_input::RemoveRoleFromDbClusterInputBuilder;

/// Fluent builder constructing a request to `RemoveRoleFromDBCluster`.
///
/// <p>Removes the asssociation of an Amazon Web Services Identity and Access Management (IAM) role from a DB cluster.</p>
/// <p>For more information on Amazon Aurora DB clusters, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/CHAP_AuroraOverview.html"> What is Amazon Aurora?</a> in the <i>Amazon Aurora User Guide</i>.</p>
/// <p>For more information on Multi-AZ DB clusters, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/multi-az-db-clusters-concepts.html"> Multi-AZ DB cluster deployments</a> in the <i>Amazon RDS User Guide.</i> </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveRoleFromDBClusterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::remove_role_from_db_cluster::builders::RemoveRoleFromDbClusterInputBuilder,
}
impl RemoveRoleFromDBClusterFluentBuilder {
    /// Creates a new `RemoveRoleFromDBCluster`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn customize_middleware(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::remove_role_from_db_cluster::RemoveRoleFromDBCluster,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_role_from_db_cluster::RemoveRoleFromDBClusterError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        ::std::result::Result::Ok(crate::client::customize::CustomizableOperation {
            handle,
            operation,
        })
    }

    // This function will go away in the near future. Do not rely on it.
    #[doc(hidden)]
    pub async fn send_middleware(
        self,
    ) -> ::std::result::Result<
        crate::operation::remove_role_from_db_cluster::RemoveRoleFromDbClusterOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_role_from_db_cluster::RemoveRoleFromDBClusterError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(::aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::remove_role_from_db_cluster::RemoveRoleFromDbClusterOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_role_from_db_cluster::RemoveRoleFromDBClusterError,
        >,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::remove_role_from_db_cluster::RemoveRoleFromDBCluster,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::remove_role_from_db_cluster::RemoveRoleFromDBClusterError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the DB cluster to disassociate the IAM role from.</p>
    pub fn db_cluster_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.db_cluster_identifier(input.into());
        self
    }
    /// <p>The name of the DB cluster to disassociate the IAM role from.</p>
    pub fn set_db_cluster_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_db_cluster_identifier(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to disassociate from the Aurora DB cluster, for example <code>arn:aws:iam::123456789012:role/AuroraAccessRole</code>.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role to disassociate from the Aurora DB cluster, for example <code>arn:aws:iam::123456789012:role/AuroraAccessRole</code>.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The name of the feature for the DB cluster that the IAM role is to be disassociated from. For information about supported feature names, see <code>DBEngineVersion</code>.</p>
    pub fn feature_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.feature_name(input.into());
        self
    }
    /// <p>The name of the feature for the DB cluster that the IAM role is to be disassociated from. For information about supported feature names, see <code>DBEngineVersion</code>.</p>
    pub fn set_feature_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_feature_name(input);
        self
    }
}
