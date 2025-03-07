// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_build::_start_build_output::StartBuildOutputBuilder;

pub use crate::operation::start_build::_start_build_input::StartBuildInputBuilder;

/// Fluent builder constructing a request to `StartBuild`.
///
/// <p>Starts running a build.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartBuildFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_build::builders::StartBuildInputBuilder,
}
impl StartBuildFluentBuilder {
    /// Creates a new `StartBuild`.
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
            crate::operation::start_build::StartBuild,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::start_build::StartBuildError>,
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
        crate::operation::start_build::StartBuildOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::start_build::StartBuildError>,
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
        crate::operation::start_build::StartBuildOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::start_build::StartBuildError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::start_build::StartBuild,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::start_build::StartBuildError>,
    > {
        self.customize_middleware().await
    }
    /// <p>The name of the CodeBuild build project to start running a build.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_name(input.into());
        self
    }
    /// <p>The name of the CodeBuild build project to start running a build.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_name(input);
        self
    }
    /// Appends an item to `secondarySourcesOverride`.
    ///
    /// To override the contents of this collection use [`set_secondary_sources_override`](Self::set_secondary_sources_override).
    ///
    /// <p> An array of <code>ProjectSource</code> objects. </p>
    pub fn secondary_sources_override(mut self, input: crate::types::ProjectSource) -> Self {
        self.inner = self.inner.secondary_sources_override(input);
        self
    }
    /// <p> An array of <code>ProjectSource</code> objects. </p>
    pub fn set_secondary_sources_override(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ProjectSource>>,
    ) -> Self {
        self.inner = self.inner.set_secondary_sources_override(input);
        self
    }
    /// Appends an item to `secondarySourcesVersionOverride`.
    ///
    /// To override the contents of this collection use [`set_secondary_sources_version_override`](Self::set_secondary_sources_version_override).
    ///
    /// <p> An array of <code>ProjectSourceVersion</code> objects that specify one or more versions of the project's secondary sources to be used for this build only. </p>
    pub fn secondary_sources_version_override(
        mut self,
        input: crate::types::ProjectSourceVersion,
    ) -> Self {
        self.inner = self.inner.secondary_sources_version_override(input);
        self
    }
    /// <p> An array of <code>ProjectSourceVersion</code> objects that specify one or more versions of the project's secondary sources to be used for this build only. </p>
    pub fn set_secondary_sources_version_override(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ProjectSourceVersion>>,
    ) -> Self {
        self.inner = self.inner.set_secondary_sources_version_override(input);
        self
    }
    /// <p>The version of the build input to be built, for this build only. If not specified, the latest version is used. If specified, the contents depends on the source provider:</p>
    /// <dl>
    /// <dt>
    /// CodeCommit
    /// </dt>
    /// <dd>
    /// <p>The commit ID, branch, or Git tag to use.</p>
    /// </dd>
    /// <dt>
    /// GitHub
    /// </dt>
    /// <dd>
    /// <p>The commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p>
    /// </dd>
    /// <dt>
    /// Bitbucket
    /// </dt>
    /// <dd>
    /// <p>The commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p>
    /// </dd>
    /// <dt>
    /// Amazon S3
    /// </dt>
    /// <dd>
    /// <p>The version ID of the object that represents the build input ZIP file to use.</p>
    /// </dd>
    /// </dl>
    /// <p>If <code>sourceVersion</code> is specified at the project level, then this <code>sourceVersion</code> (at the build level) takes precedence. </p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>CodeBuild User Guide</i>. </p>
    pub fn source_version(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.source_version(input.into());
        self
    }
    /// <p>The version of the build input to be built, for this build only. If not specified, the latest version is used. If specified, the contents depends on the source provider:</p>
    /// <dl>
    /// <dt>
    /// CodeCommit
    /// </dt>
    /// <dd>
    /// <p>The commit ID, branch, or Git tag to use.</p>
    /// </dd>
    /// <dt>
    /// GitHub
    /// </dt>
    /// <dd>
    /// <p>The commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p>
    /// </dd>
    /// <dt>
    /// Bitbucket
    /// </dt>
    /// <dd>
    /// <p>The commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p>
    /// </dd>
    /// <dt>
    /// Amazon S3
    /// </dt>
    /// <dd>
    /// <p>The version ID of the object that represents the build input ZIP file to use.</p>
    /// </dd>
    /// </dl>
    /// <p>If <code>sourceVersion</code> is specified at the project level, then this <code>sourceVersion</code> (at the build level) takes precedence. </p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>CodeBuild User Guide</i>. </p>
    pub fn set_source_version(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_version(input);
        self
    }
    /// <p>Build output artifact settings that override, for this build only, the latest ones already defined in the build project.</p>
    pub fn artifacts_override(mut self, input: crate::types::ProjectArtifacts) -> Self {
        self.inner = self.inner.artifacts_override(input);
        self
    }
    /// <p>Build output artifact settings that override, for this build only, the latest ones already defined in the build project.</p>
    pub fn set_artifacts_override(
        mut self,
        input: ::std::option::Option<crate::types::ProjectArtifacts>,
    ) -> Self {
        self.inner = self.inner.set_artifacts_override(input);
        self
    }
    /// Appends an item to `secondaryArtifactsOverride`.
    ///
    /// To override the contents of this collection use [`set_secondary_artifacts_override`](Self::set_secondary_artifacts_override).
    ///
    /// <p> An array of <code>ProjectArtifacts</code> objects. </p>
    pub fn secondary_artifacts_override(mut self, input: crate::types::ProjectArtifacts) -> Self {
        self.inner = self.inner.secondary_artifacts_override(input);
        self
    }
    /// <p> An array of <code>ProjectArtifacts</code> objects. </p>
    pub fn set_secondary_artifacts_override(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ProjectArtifacts>>,
    ) -> Self {
        self.inner = self.inner.set_secondary_artifacts_override(input);
        self
    }
    /// Appends an item to `environmentVariablesOverride`.
    ///
    /// To override the contents of this collection use [`set_environment_variables_override`](Self::set_environment_variables_override).
    ///
    /// <p>A set of environment variables that overrides, for this build only, the latest ones already defined in the build project.</p>
    pub fn environment_variables_override(
        mut self,
        input: crate::types::EnvironmentVariable,
    ) -> Self {
        self.inner = self.inner.environment_variables_override(input);
        self
    }
    /// <p>A set of environment variables that overrides, for this build only, the latest ones already defined in the build project.</p>
    pub fn set_environment_variables_override(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::EnvironmentVariable>>,
    ) -> Self {
        self.inner = self.inner.set_environment_variables_override(input);
        self
    }
    /// <p>A source input type, for this build, that overrides the source input defined in the build project.</p>
    pub fn source_type_override(mut self, input: crate::types::SourceType) -> Self {
        self.inner = self.inner.source_type_override(input);
        self
    }
    /// <p>A source input type, for this build, that overrides the source input defined in the build project.</p>
    pub fn set_source_type_override(
        mut self,
        input: ::std::option::Option<crate::types::SourceType>,
    ) -> Self {
        self.inner = self.inner.set_source_type_override(input);
        self
    }
    /// <p>A location that overrides, for this build, the source location for the one defined in the build project.</p>
    pub fn source_location_override(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.source_location_override(input.into());
        self
    }
    /// <p>A location that overrides, for this build, the source location for the one defined in the build project.</p>
    pub fn set_source_location_override(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_location_override(input);
        self
    }
    /// <p>An authorization type for this build that overrides the one defined in the build project. This override applies only if the build project's source is BitBucket or GitHub.</p>
    pub fn source_auth_override(mut self, input: crate::types::SourceAuth) -> Self {
        self.inner = self.inner.source_auth_override(input);
        self
    }
    /// <p>An authorization type for this build that overrides the one defined in the build project. This override applies only if the build project's source is BitBucket or GitHub.</p>
    pub fn set_source_auth_override(
        mut self,
        input: ::std::option::Option<crate::types::SourceAuth>,
    ) -> Self {
        self.inner = self.inner.set_source_auth_override(input);
        self
    }
    /// <p>The user-defined depth of history, with a minimum value of 0, that overrides, for this build only, any previous depth of history defined in the build project.</p>
    pub fn git_clone_depth_override(mut self, input: i32) -> Self {
        self.inner = self.inner.git_clone_depth_override(input);
        self
    }
    /// <p>The user-defined depth of history, with a minimum value of 0, that overrides, for this build only, any previous depth of history defined in the build project.</p>
    pub fn set_git_clone_depth_override(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_git_clone_depth_override(input);
        self
    }
    /// <p> Information about the Git submodules configuration for this build of an CodeBuild build project. </p>
    pub fn git_submodules_config_override(
        mut self,
        input: crate::types::GitSubmodulesConfig,
    ) -> Self {
        self.inner = self.inner.git_submodules_config_override(input);
        self
    }
    /// <p> Information about the Git submodules configuration for this build of an CodeBuild build project. </p>
    pub fn set_git_submodules_config_override(
        mut self,
        input: ::std::option::Option<crate::types::GitSubmodulesConfig>,
    ) -> Self {
        self.inner = self.inner.set_git_submodules_config_override(input);
        self
    }
    /// <p>A buildspec file declaration that overrides, for this build only, the latest one already defined in the build project.</p>
    /// <p> If this value is set, it can be either an inline buildspec definition, the path to an alternate buildspec file relative to the value of the built-in <code>CODEBUILD_SRC_DIR</code> environment variable, or the path to an S3 bucket. The bucket must be in the same Amazon Web Services Region as the build project. Specify the buildspec file using its ARN (for example, <code>arn:aws:s3:::my-codebuild-sample2/buildspec.yml</code>). If this value is not provided or is set to an empty string, the source code must contain a buildspec file in its root directory. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-spec-ref.html#build-spec-ref-name-storage">Buildspec File Name and Storage Location</a>. </p>
    pub fn buildspec_override(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.buildspec_override(input.into());
        self
    }
    /// <p>A buildspec file declaration that overrides, for this build only, the latest one already defined in the build project.</p>
    /// <p> If this value is set, it can be either an inline buildspec definition, the path to an alternate buildspec file relative to the value of the built-in <code>CODEBUILD_SRC_DIR</code> environment variable, or the path to an S3 bucket. The bucket must be in the same Amazon Web Services Region as the build project. Specify the buildspec file using its ARN (for example, <code>arn:aws:s3:::my-codebuild-sample2/buildspec.yml</code>). If this value is not provided or is set to an empty string, the source code must contain a buildspec file in its root directory. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/build-spec-ref.html#build-spec-ref-name-storage">Buildspec File Name and Storage Location</a>. </p>
    pub fn set_buildspec_override(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_buildspec_override(input);
        self
    }
    /// <p>Enable this flag to override the insecure SSL setting that is specified in the build project. The insecure SSL setting determines whether to ignore SSL warnings while connecting to the project source code. This override applies only if the build's source is GitHub Enterprise.</p>
    pub fn insecure_ssl_override(mut self, input: bool) -> Self {
        self.inner = self.inner.insecure_ssl_override(input);
        self
    }
    /// <p>Enable this flag to override the insecure SSL setting that is specified in the build project. The insecure SSL setting determines whether to ignore SSL warnings while connecting to the project source code. This override applies only if the build's source is GitHub Enterprise.</p>
    pub fn set_insecure_ssl_override(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_insecure_ssl_override(input);
        self
    }
    /// <p> Set to true to report to your source provider the status of a build's start and completion. If you use this option with a source provider other than GitHub, GitHub Enterprise, or Bitbucket, an <code>invalidInputException</code> is thrown. </p>
    /// <p>To be able to report the build status to the source provider, the user associated with the source provider must have write access to the repo. If the user does not have write access, the build status cannot be updated. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/access-tokens.html">Source provider access</a> in the <i>CodeBuild User Guide</i>.</p> <note>
    /// <p> The status of a build triggered by a webhook is always reported to your source provider. </p>
    /// </note>
    pub fn report_build_status_override(mut self, input: bool) -> Self {
        self.inner = self.inner.report_build_status_override(input);
        self
    }
    /// <p> Set to true to report to your source provider the status of a build's start and completion. If you use this option with a source provider other than GitHub, GitHub Enterprise, or Bitbucket, an <code>invalidInputException</code> is thrown. </p>
    /// <p>To be able to report the build status to the source provider, the user associated with the source provider must have write access to the repo. If the user does not have write access, the build status cannot be updated. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/access-tokens.html">Source provider access</a> in the <i>CodeBuild User Guide</i>.</p> <note>
    /// <p> The status of a build triggered by a webhook is always reported to your source provider. </p>
    /// </note>
    pub fn set_report_build_status_override(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_report_build_status_override(input);
        self
    }
    /// <p>Contains information that defines how the build project reports the build status to the source provider. This option is only used when the source provider is <code>GITHUB</code>, <code>GITHUB_ENTERPRISE</code>, or <code>BITBUCKET</code>.</p>
    pub fn build_status_config_override(mut self, input: crate::types::BuildStatusConfig) -> Self {
        self.inner = self.inner.build_status_config_override(input);
        self
    }
    /// <p>Contains information that defines how the build project reports the build status to the source provider. This option is only used when the source provider is <code>GITHUB</code>, <code>GITHUB_ENTERPRISE</code>, or <code>BITBUCKET</code>.</p>
    pub fn set_build_status_config_override(
        mut self,
        input: ::std::option::Option<crate::types::BuildStatusConfig>,
    ) -> Self {
        self.inner = self.inner.set_build_status_config_override(input);
        self
    }
    /// <p>A container type for this build that overrides the one specified in the build project.</p>
    pub fn environment_type_override(mut self, input: crate::types::EnvironmentType) -> Self {
        self.inner = self.inner.environment_type_override(input);
        self
    }
    /// <p>A container type for this build that overrides the one specified in the build project.</p>
    pub fn set_environment_type_override(
        mut self,
        input: ::std::option::Option<crate::types::EnvironmentType>,
    ) -> Self {
        self.inner = self.inner.set_environment_type_override(input);
        self
    }
    /// <p>The name of an image for this build that overrides the one specified in the build project.</p>
    pub fn image_override(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.image_override(input.into());
        self
    }
    /// <p>The name of an image for this build that overrides the one specified in the build project.</p>
    pub fn set_image_override(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_image_override(input);
        self
    }
    /// <p>The name of a compute type for this build that overrides the one specified in the build project.</p>
    pub fn compute_type_override(mut self, input: crate::types::ComputeType) -> Self {
        self.inner = self.inner.compute_type_override(input);
        self
    }
    /// <p>The name of a compute type for this build that overrides the one specified in the build project.</p>
    pub fn set_compute_type_override(
        mut self,
        input: ::std::option::Option<crate::types::ComputeType>,
    ) -> Self {
        self.inner = self.inner.set_compute_type_override(input);
        self
    }
    /// <p>The name of a certificate for this build that overrides the one specified in the build project.</p>
    pub fn certificate_override(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.certificate_override(input.into());
        self
    }
    /// <p>The name of a certificate for this build that overrides the one specified in the build project.</p>
    pub fn set_certificate_override(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_certificate_override(input);
        self
    }
    /// <p>A ProjectCache object specified for this build that overrides the one defined in the build project.</p>
    pub fn cache_override(mut self, input: crate::types::ProjectCache) -> Self {
        self.inner = self.inner.cache_override(input);
        self
    }
    /// <p>A ProjectCache object specified for this build that overrides the one defined in the build project.</p>
    pub fn set_cache_override(
        mut self,
        input: ::std::option::Option<crate::types::ProjectCache>,
    ) -> Self {
        self.inner = self.inner.set_cache_override(input);
        self
    }
    /// <p>The name of a service role for this build that overrides the one specified in the build project.</p>
    pub fn service_role_override(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.service_role_override(input.into());
        self
    }
    /// <p>The name of a service role for this build that overrides the one specified in the build project.</p>
    pub fn set_service_role_override(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_service_role_override(input);
        self
    }
    /// <p>Enable this flag to override privileged mode in the build project.</p>
    pub fn privileged_mode_override(mut self, input: bool) -> Self {
        self.inner = self.inner.privileged_mode_override(input);
        self
    }
    /// <p>Enable this flag to override privileged mode in the build project.</p>
    pub fn set_privileged_mode_override(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_privileged_mode_override(input);
        self
    }
    /// <p>The number of build timeout minutes, from 5 to 480 (8 hours), that overrides, for this build only, the latest setting already defined in the build project.</p>
    pub fn timeout_in_minutes_override(mut self, input: i32) -> Self {
        self.inner = self.inner.timeout_in_minutes_override(input);
        self
    }
    /// <p>The number of build timeout minutes, from 5 to 480 (8 hours), that overrides, for this build only, the latest setting already defined in the build project.</p>
    pub fn set_timeout_in_minutes_override(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_timeout_in_minutes_override(input);
        self
    }
    /// <p> The number of minutes a build is allowed to be queued before it times out. </p>
    pub fn queued_timeout_in_minutes_override(mut self, input: i32) -> Self {
        self.inner = self.inner.queued_timeout_in_minutes_override(input);
        self
    }
    /// <p> The number of minutes a build is allowed to be queued before it times out. </p>
    pub fn set_queued_timeout_in_minutes_override(
        mut self,
        input: ::std::option::Option<i32>,
    ) -> Self {
        self.inner = self.inner.set_queued_timeout_in_minutes_override(input);
        self
    }
    /// <p>The Key Management Service customer master key (CMK) that overrides the one specified in the build project. The CMK key encrypts the build output artifacts.</p> <note>
    /// <p> You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key. </p>
    /// </note>
    /// <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/
    /// <alias-name></alias-name></code>).</p>
    pub fn encryption_key_override(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.encryption_key_override(input.into());
        self
    }
    /// <p>The Key Management Service customer master key (CMK) that overrides the one specified in the build project. The CMK key encrypts the build output artifacts.</p> <note>
    /// <p> You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key. </p>
    /// </note>
    /// <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/
    /// <alias-name></alias-name></code>).</p>
    pub fn set_encryption_key_override(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_encryption_key_override(input);
        self
    }
    /// <p>A unique, case sensitive identifier you provide to ensure the idempotency of the StartBuild request. The token is included in the StartBuild request and is valid for 5 minutes. If you repeat the StartBuild request with the same token, but change a parameter, CodeBuild returns a parameter mismatch error. </p>
    pub fn idempotency_token(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.idempotency_token(input.into());
        self
    }
    /// <p>A unique, case sensitive identifier you provide to ensure the idempotency of the StartBuild request. The token is included in the StartBuild request and is valid for 5 minutes. If you repeat the StartBuild request with the same token, but change a parameter, CodeBuild returns a parameter mismatch error. </p>
    pub fn set_idempotency_token(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_idempotency_token(input);
        self
    }
    /// <p> Log settings for this build that override the log settings defined in the build project. </p>
    pub fn logs_config_override(mut self, input: crate::types::LogsConfig) -> Self {
        self.inner = self.inner.logs_config_override(input);
        self
    }
    /// <p> Log settings for this build that override the log settings defined in the build project. </p>
    pub fn set_logs_config_override(
        mut self,
        input: ::std::option::Option<crate::types::LogsConfig>,
    ) -> Self {
        self.inner = self.inner.set_logs_config_override(input);
        self
    }
    /// <p> The credentials for access to a private registry. </p>
    pub fn registry_credential_override(mut self, input: crate::types::RegistryCredential) -> Self {
        self.inner = self.inner.registry_credential_override(input);
        self
    }
    /// <p> The credentials for access to a private registry. </p>
    pub fn set_registry_credential_override(
        mut self,
        input: ::std::option::Option<crate::types::RegistryCredential>,
    ) -> Self {
        self.inner = self.inner.set_registry_credential_override(input);
        self
    }
    /// <p>The type of credentials CodeBuild uses to pull images in your build. There are two valid values: </p>
    /// <dl>
    /// <dt>
    /// CODEBUILD
    /// </dt>
    /// <dd>
    /// <p>Specifies that CodeBuild uses its own credentials. This requires that you modify your ECR repository policy to trust CodeBuild's service principal.</p>
    /// </dd>
    /// <dt>
    /// SERVICE_ROLE
    /// </dt>
    /// <dd>
    /// <p>Specifies that CodeBuild uses your build project's service role. </p>
    /// </dd>
    /// </dl>
    /// <p>When using a cross-account or private registry image, you must use <code>SERVICE_ROLE</code> credentials. When using an CodeBuild curated image, you must use <code>CODEBUILD</code> credentials. </p>
    pub fn image_pull_credentials_type_override(
        mut self,
        input: crate::types::ImagePullCredentialsType,
    ) -> Self {
        self.inner = self.inner.image_pull_credentials_type_override(input);
        self
    }
    /// <p>The type of credentials CodeBuild uses to pull images in your build. There are two valid values: </p>
    /// <dl>
    /// <dt>
    /// CODEBUILD
    /// </dt>
    /// <dd>
    /// <p>Specifies that CodeBuild uses its own credentials. This requires that you modify your ECR repository policy to trust CodeBuild's service principal.</p>
    /// </dd>
    /// <dt>
    /// SERVICE_ROLE
    /// </dt>
    /// <dd>
    /// <p>Specifies that CodeBuild uses your build project's service role. </p>
    /// </dd>
    /// </dl>
    /// <p>When using a cross-account or private registry image, you must use <code>SERVICE_ROLE</code> credentials. When using an CodeBuild curated image, you must use <code>CODEBUILD</code> credentials. </p>
    pub fn set_image_pull_credentials_type_override(
        mut self,
        input: ::std::option::Option<crate::types::ImagePullCredentialsType>,
    ) -> Self {
        self.inner = self.inner.set_image_pull_credentials_type_override(input);
        self
    }
    /// <p>Specifies if session debugging is enabled for this build. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/session-manager.html">Viewing a running build in Session Manager</a>.</p>
    pub fn debug_session_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.debug_session_enabled(input);
        self
    }
    /// <p>Specifies if session debugging is enabled for this build. For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/session-manager.html">Viewing a running build in Session Manager</a>.</p>
    pub fn set_debug_session_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_debug_session_enabled(input);
        self
    }
}
