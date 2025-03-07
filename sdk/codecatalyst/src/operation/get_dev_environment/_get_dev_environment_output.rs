// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetDevEnvironmentOutput {
    /// <p>The name of the space.</p>
    #[doc(hidden)]
    pub space_name: ::std::option::Option<::std::string::String>,
    /// <p>The name of the project in the space.</p>
    #[doc(hidden)]
    pub project_name: ::std::option::Option<::std::string::String>,
    /// <p>The system-generated unique ID of the Dev Environment. </p>
    #[doc(hidden)]
    pub id: ::std::option::Option<::std::string::String>,
    /// <p>The time when the Dev Environment was last updated, in coordinated universal time (UTC) timestamp format as specified in <a href="https://www.rfc-editor.org/rfc/rfc3339#section-5.6">RFC 3339</a>.</p>
    #[doc(hidden)]
    pub last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The system-generated unique ID of the user who created the Dev Environment. </p>
    #[doc(hidden)]
    pub creator_id: ::std::option::Option<::std::string::String>,
    /// <p>The current status of the Dev Environment.</p>
    #[doc(hidden)]
    pub status: ::std::option::Option<crate::types::DevEnvironmentStatus>,
    /// <p>The reason for the status.</p>
    #[doc(hidden)]
    pub status_reason: ::std::option::Option<::std::string::String>,
    /// <p>The source repository that contains the branch cloned into the Dev Environment. </p>
    #[doc(hidden)]
    pub repositories:
        ::std::option::Option<::std::vec::Vec<crate::types::DevEnvironmentRepositorySummary>>,
    /// <p>The user-specified alias for the Dev Environment. </p>
    #[doc(hidden)]
    pub alias: ::std::option::Option<::std::string::String>,
    /// <p>Information about the integrated development environment (IDE) configured for the Dev Environment. </p>
    #[doc(hidden)]
    pub ides: ::std::option::Option<::std::vec::Vec<crate::types::Ide>>,
    /// <p>The Amazon EC2 instace type to use for the Dev Environment. </p>
    #[doc(hidden)]
    pub instance_type: ::std::option::Option<crate::types::InstanceType>,
    /// <p>The amount of time the Dev Environment will run without any activity detected before stopping, in minutes.</p>
    #[doc(hidden)]
    pub inactivity_timeout_minutes: i32,
    /// <p>Information about the amount of storage allocated to the Dev Environment. By default, a Dev Environment is configured to have 16GB of persistent storage.</p>
    #[doc(hidden)]
    pub persistent_storage: ::std::option::Option<crate::types::PersistentStorage>,
    _request_id: Option<String>,
}
impl GetDevEnvironmentOutput {
    /// <p>The name of the space.</p>
    pub fn space_name(&self) -> ::std::option::Option<&str> {
        self.space_name.as_deref()
    }
    /// <p>The name of the project in the space.</p>
    pub fn project_name(&self) -> ::std::option::Option<&str> {
        self.project_name.as_deref()
    }
    /// <p>The system-generated unique ID of the Dev Environment. </p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
    /// <p>The time when the Dev Environment was last updated, in coordinated universal time (UTC) timestamp format as specified in <a href="https://www.rfc-editor.org/rfc/rfc3339#section-5.6">RFC 3339</a>.</p>
    pub fn last_updated_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.last_updated_time.as_ref()
    }
    /// <p>The system-generated unique ID of the user who created the Dev Environment. </p>
    pub fn creator_id(&self) -> ::std::option::Option<&str> {
        self.creator_id.as_deref()
    }
    /// <p>The current status of the Dev Environment.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::DevEnvironmentStatus> {
        self.status.as_ref()
    }
    /// <p>The reason for the status.</p>
    pub fn status_reason(&self) -> ::std::option::Option<&str> {
        self.status_reason.as_deref()
    }
    /// <p>The source repository that contains the branch cloned into the Dev Environment. </p>
    pub fn repositories(
        &self,
    ) -> ::std::option::Option<&[crate::types::DevEnvironmentRepositorySummary]> {
        self.repositories.as_deref()
    }
    /// <p>The user-specified alias for the Dev Environment. </p>
    pub fn alias(&self) -> ::std::option::Option<&str> {
        self.alias.as_deref()
    }
    /// <p>Information about the integrated development environment (IDE) configured for the Dev Environment. </p>
    pub fn ides(&self) -> ::std::option::Option<&[crate::types::Ide]> {
        self.ides.as_deref()
    }
    /// <p>The Amazon EC2 instace type to use for the Dev Environment. </p>
    pub fn instance_type(&self) -> ::std::option::Option<&crate::types::InstanceType> {
        self.instance_type.as_ref()
    }
    /// <p>The amount of time the Dev Environment will run without any activity detected before stopping, in minutes.</p>
    pub fn inactivity_timeout_minutes(&self) -> i32 {
        self.inactivity_timeout_minutes
    }
    /// <p>Information about the amount of storage allocated to the Dev Environment. By default, a Dev Environment is configured to have 16GB of persistent storage.</p>
    pub fn persistent_storage(&self) -> ::std::option::Option<&crate::types::PersistentStorage> {
        self.persistent_storage.as_ref()
    }
}
impl ::aws_http::request_id::RequestId for GetDevEnvironmentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetDevEnvironmentOutput {
    /// Creates a new builder-style object to manufacture [`GetDevEnvironmentOutput`](crate::operation::get_dev_environment::GetDevEnvironmentOutput).
    pub fn builder(
    ) -> crate::operation::get_dev_environment::builders::GetDevEnvironmentOutputBuilder {
        crate::operation::get_dev_environment::builders::GetDevEnvironmentOutputBuilder::default()
    }
}

/// A builder for [`GetDevEnvironmentOutput`](crate::operation::get_dev_environment::GetDevEnvironmentOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetDevEnvironmentOutputBuilder {
    pub(crate) space_name: ::std::option::Option<::std::string::String>,
    pub(crate) project_name: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) last_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) creator_id: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<crate::types::DevEnvironmentStatus>,
    pub(crate) status_reason: ::std::option::Option<::std::string::String>,
    pub(crate) repositories:
        ::std::option::Option<::std::vec::Vec<crate::types::DevEnvironmentRepositorySummary>>,
    pub(crate) alias: ::std::option::Option<::std::string::String>,
    pub(crate) ides: ::std::option::Option<::std::vec::Vec<crate::types::Ide>>,
    pub(crate) instance_type: ::std::option::Option<crate::types::InstanceType>,
    pub(crate) inactivity_timeout_minutes: ::std::option::Option<i32>,
    pub(crate) persistent_storage: ::std::option::Option<crate::types::PersistentStorage>,
    _request_id: Option<String>,
}
impl GetDevEnvironmentOutputBuilder {
    /// <p>The name of the space.</p>
    pub fn space_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.space_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the space.</p>
    pub fn set_space_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.space_name = input;
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn project_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.project_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the project in the space.</p>
    pub fn set_project_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.project_name = input;
        self
    }
    /// <p>The system-generated unique ID of the Dev Environment. </p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The system-generated unique ID of the Dev Environment. </p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The time when the Dev Environment was last updated, in coordinated universal time (UTC) timestamp format as specified in <a href="https://www.rfc-editor.org/rfc/rfc3339#section-5.6">RFC 3339</a>.</p>
    pub fn last_updated_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when the Dev Environment was last updated, in coordinated universal time (UTC) timestamp format as specified in <a href="https://www.rfc-editor.org/rfc/rfc3339#section-5.6">RFC 3339</a>.</p>
    pub fn set_last_updated_time(
        mut self,
        input: ::std::option::Option<::aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_updated_time = input;
        self
    }
    /// <p>The system-generated unique ID of the user who created the Dev Environment. </p>
    pub fn creator_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.creator_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The system-generated unique ID of the user who created the Dev Environment. </p>
    pub fn set_creator_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.creator_id = input;
        self
    }
    /// <p>The current status of the Dev Environment.</p>
    pub fn status(mut self, input: crate::types::DevEnvironmentStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the Dev Environment.</p>
    pub fn set_status(
        mut self,
        input: ::std::option::Option<crate::types::DevEnvironmentStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>The reason for the status.</p>
    pub fn status_reason(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for the status.</p>
    pub fn set_status_reason(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.status_reason = input;
        self
    }
    /// Appends an item to `repositories`.
    ///
    /// To override the contents of this collection use [`set_repositories`](Self::set_repositories).
    ///
    /// <p>The source repository that contains the branch cloned into the Dev Environment. </p>
    pub fn repositories(mut self, input: crate::types::DevEnvironmentRepositorySummary) -> Self {
        let mut v = self.repositories.unwrap_or_default();
        v.push(input);
        self.repositories = ::std::option::Option::Some(v);
        self
    }
    /// <p>The source repository that contains the branch cloned into the Dev Environment. </p>
    pub fn set_repositories(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<crate::types::DevEnvironmentRepositorySummary>,
        >,
    ) -> Self {
        self.repositories = input;
        self
    }
    /// <p>The user-specified alias for the Dev Environment. </p>
    pub fn alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.alias = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user-specified alias for the Dev Environment. </p>
    pub fn set_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.alias = input;
        self
    }
    /// Appends an item to `ides`.
    ///
    /// To override the contents of this collection use [`set_ides`](Self::set_ides).
    ///
    /// <p>Information about the integrated development environment (IDE) configured for the Dev Environment. </p>
    pub fn ides(mut self, input: crate::types::Ide) -> Self {
        let mut v = self.ides.unwrap_or_default();
        v.push(input);
        self.ides = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the integrated development environment (IDE) configured for the Dev Environment. </p>
    pub fn set_ides(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Ide>>,
    ) -> Self {
        self.ides = input;
        self
    }
    /// <p>The Amazon EC2 instace type to use for the Dev Environment. </p>
    pub fn instance_type(mut self, input: crate::types::InstanceType) -> Self {
        self.instance_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Amazon EC2 instace type to use for the Dev Environment. </p>
    pub fn set_instance_type(
        mut self,
        input: ::std::option::Option<crate::types::InstanceType>,
    ) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The amount of time the Dev Environment will run without any activity detected before stopping, in minutes.</p>
    pub fn inactivity_timeout_minutes(mut self, input: i32) -> Self {
        self.inactivity_timeout_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of time the Dev Environment will run without any activity detected before stopping, in minutes.</p>
    pub fn set_inactivity_timeout_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inactivity_timeout_minutes = input;
        self
    }
    /// <p>Information about the amount of storage allocated to the Dev Environment. By default, a Dev Environment is configured to have 16GB of persistent storage.</p>
    pub fn persistent_storage(mut self, input: crate::types::PersistentStorage) -> Self {
        self.persistent_storage = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the amount of storage allocated to the Dev Environment. By default, a Dev Environment is configured to have 16GB of persistent storage.</p>
    pub fn set_persistent_storage(
        mut self,
        input: ::std::option::Option<crate::types::PersistentStorage>,
    ) -> Self {
        self.persistent_storage = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetDevEnvironmentOutput`](crate::operation::get_dev_environment::GetDevEnvironmentOutput).
    pub fn build(self) -> crate::operation::get_dev_environment::GetDevEnvironmentOutput {
        crate::operation::get_dev_environment::GetDevEnvironmentOutput {
            space_name: self.space_name,
            project_name: self.project_name,
            id: self.id,
            last_updated_time: self.last_updated_time,
            creator_id: self.creator_id,
            status: self.status,
            status_reason: self.status_reason,
            repositories: self.repositories,
            alias: self.alias,
            ides: self.ides,
            instance_type: self.instance_type,
            inactivity_timeout_minutes: self.inactivity_timeout_minutes.unwrap_or_default(),
            persistent_storage: self.persistent_storage,
            _request_id: self._request_id,
        }
    }
}
