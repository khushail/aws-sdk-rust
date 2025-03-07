// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateAnalysisPermissionsInput {
    /// <p>The ID of the Amazon Web Services account that contains the analysis whose permissions you're updating. You must be using the Amazon Web Services account that the analysis is in.</p>
    #[doc(hidden)]
    pub aws_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the analysis whose permissions you're updating. The ID is part of the analysis URL.</p>
    #[doc(hidden)]
    pub analysis_id: ::std::option::Option<::std::string::String>,
    /// <p>A structure that describes the permissions to add and the principal to add them to.</p>
    #[doc(hidden)]
    pub grant_permissions: ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>,
    /// <p>A structure that describes the permissions to remove and the principal to remove them from.</p>
    #[doc(hidden)]
    pub revoke_permissions:
        ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>,
}
impl UpdateAnalysisPermissionsInput {
    /// <p>The ID of the Amazon Web Services account that contains the analysis whose permissions you're updating. You must be using the Amazon Web Services account that the analysis is in.</p>
    pub fn aws_account_id(&self) -> ::std::option::Option<&str> {
        self.aws_account_id.as_deref()
    }
    /// <p>The ID of the analysis whose permissions you're updating. The ID is part of the analysis URL.</p>
    pub fn analysis_id(&self) -> ::std::option::Option<&str> {
        self.analysis_id.as_deref()
    }
    /// <p>A structure that describes the permissions to add and the principal to add them to.</p>
    pub fn grant_permissions(&self) -> ::std::option::Option<&[crate::types::ResourcePermission]> {
        self.grant_permissions.as_deref()
    }
    /// <p>A structure that describes the permissions to remove and the principal to remove them from.</p>
    pub fn revoke_permissions(&self) -> ::std::option::Option<&[crate::types::ResourcePermission]> {
        self.revoke_permissions.as_deref()
    }
}
impl UpdateAnalysisPermissionsInput {
    /// Creates a new builder-style object to manufacture [`UpdateAnalysisPermissionsInput`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsInput).
    pub fn builder() -> crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsInputBuilder{
        crate::operation::update_analysis_permissions::builders::UpdateAnalysisPermissionsInputBuilder::default()
    }
}

/// A builder for [`UpdateAnalysisPermissionsInput`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateAnalysisPermissionsInputBuilder {
    pub(crate) aws_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) analysis_id: ::std::option::Option<::std::string::String>,
    pub(crate) grant_permissions:
        ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>,
    pub(crate) revoke_permissions:
        ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>,
}
impl UpdateAnalysisPermissionsInputBuilder {
    /// <p>The ID of the Amazon Web Services account that contains the analysis whose permissions you're updating. You must be using the Amazon Web Services account that the analysis is in.</p>
    pub fn aws_account_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.aws_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the analysis whose permissions you're updating. You must be using the Amazon Web Services account that the analysis is in.</p>
    pub fn set_aws_account_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.aws_account_id = input;
        self
    }
    /// <p>The ID of the analysis whose permissions you're updating. The ID is part of the analysis URL.</p>
    pub fn analysis_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.analysis_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the analysis whose permissions you're updating. The ID is part of the analysis URL.</p>
    pub fn set_analysis_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.analysis_id = input;
        self
    }
    /// Appends an item to `grant_permissions`.
    ///
    /// To override the contents of this collection use [`set_grant_permissions`](Self::set_grant_permissions).
    ///
    /// <p>A structure that describes the permissions to add and the principal to add them to.</p>
    pub fn grant_permissions(mut self, input: crate::types::ResourcePermission) -> Self {
        let mut v = self.grant_permissions.unwrap_or_default();
        v.push(input);
        self.grant_permissions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A structure that describes the permissions to add and the principal to add them to.</p>
    pub fn set_grant_permissions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>,
    ) -> Self {
        self.grant_permissions = input;
        self
    }
    /// Appends an item to `revoke_permissions`.
    ///
    /// To override the contents of this collection use [`set_revoke_permissions`](Self::set_revoke_permissions).
    ///
    /// <p>A structure that describes the permissions to remove and the principal to remove them from.</p>
    pub fn revoke_permissions(mut self, input: crate::types::ResourcePermission) -> Self {
        let mut v = self.revoke_permissions.unwrap_or_default();
        v.push(input);
        self.revoke_permissions = ::std::option::Option::Some(v);
        self
    }
    /// <p>A structure that describes the permissions to remove and the principal to remove them from.</p>
    pub fn set_revoke_permissions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>,
    ) -> Self {
        self.revoke_permissions = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateAnalysisPermissionsInput`](crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_analysis_permissions::UpdateAnalysisPermissionsInput {
                aws_account_id: self.aws_account_id,
                analysis_id: self.analysis_id,
                grant_permissions: self.grant_permissions,
                revoke_permissions: self.revoke_permissions,
            },
        )
    }
}
