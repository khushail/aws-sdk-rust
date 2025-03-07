// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_dataset_group::_create_dataset_group_output::CreateDatasetGroupOutputBuilder;

pub use crate::operation::create_dataset_group::_create_dataset_group_input::CreateDatasetGroupInputBuilder;

/// Fluent builder constructing a request to `CreateDatasetGroup`.
///
/// <p>Creates a dataset group, which holds a collection of related datasets. You can add datasets to the dataset group when you create the dataset group, or later by using the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_UpdateDatasetGroup.html">UpdateDatasetGroup</a> operation.</p>
/// <p>After creating a dataset group and adding datasets, you use the dataset group when you create a predictor. For more information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-datasets-groups.html">Dataset groups</a>.</p>
/// <p>To get a list of all your datasets groups, use the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_ListDatasetGroups.html">ListDatasetGroups</a> operation.</p> <note>
/// <p>The <code>Status</code> of a dataset group must be <code>ACTIVE</code> before you can use the dataset group to create a predictor. To get the status, use the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_DescribeDatasetGroup.html">DescribeDatasetGroup</a> operation.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateDatasetGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_dataset_group::builders::CreateDatasetGroupInputBuilder,
}
impl CreateDatasetGroupFluentBuilder {
    /// Creates a new `CreateDatasetGroup`.
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
            crate::operation::create_dataset_group::CreateDatasetGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_dataset_group::CreateDatasetGroupError,
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
        crate::operation::create_dataset_group::CreateDatasetGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_dataset_group::CreateDatasetGroupError,
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
        crate::operation::create_dataset_group::CreateDatasetGroupOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_dataset_group::CreateDatasetGroupError,
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
            crate::operation::create_dataset_group::CreateDatasetGroup,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::create_dataset_group::CreateDatasetGroupError,
        >,
    > {
        self.customize_middleware().await
    }
    /// <p>A name for the dataset group.</p>
    pub fn dataset_group_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.dataset_group_name(input.into());
        self
    }
    /// <p>A name for the dataset group.</p>
    pub fn set_dataset_group_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_dataset_group_name(input);
        self
    }
    /// <p>The domain associated with the dataset group. When you add a dataset to a dataset group, this value and the value specified for the <code>Domain</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDataset.html">CreateDataset</a> operation must match.</p>
    /// <p>The <code>Domain</code> and <code>DatasetType</code> that you choose determine the fields that must be present in training data that you import to a dataset. For example, if you choose the <code>RETAIL</code> domain and <code>TARGET_TIME_SERIES</code> as the <code>DatasetType</code>, Amazon Forecast requires that <code>item_id</code>, <code>timestamp</code>, and <code>demand</code> fields are present in your data. For more information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-datasets-groups.html">Dataset groups</a>.</p>
    pub fn domain(mut self, input: crate::types::Domain) -> Self {
        self.inner = self.inner.domain(input);
        self
    }
    /// <p>The domain associated with the dataset group. When you add a dataset to a dataset group, this value and the value specified for the <code>Domain</code> parameter of the <a href="https://docs.aws.amazon.com/forecast/latest/dg/API_CreateDataset.html">CreateDataset</a> operation must match.</p>
    /// <p>The <code>Domain</code> and <code>DatasetType</code> that you choose determine the fields that must be present in training data that you import to a dataset. For example, if you choose the <code>RETAIL</code> domain and <code>TARGET_TIME_SERIES</code> as the <code>DatasetType</code>, Amazon Forecast requires that <code>item_id</code>, <code>timestamp</code>, and <code>demand</code> fields are present in your data. For more information, see <a href="https://docs.aws.amazon.com/forecast/latest/dg/howitworks-datasets-groups.html">Dataset groups</a>.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<crate::types::Domain>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// Appends an item to `DatasetArns`.
    ///
    /// To override the contents of this collection use [`set_dataset_arns`](Self::set_dataset_arns).
    ///
    /// <p>An array of Amazon Resource Names (ARNs) of the datasets that you want to include in the dataset group.</p>
    pub fn dataset_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dataset_arns(input.into());
        self
    }
    /// <p>An array of Amazon Resource Names (ARNs) of the datasets that you want to include in the dataset group.</p>
    pub fn set_dataset_arns(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_dataset_arns(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The optional metadata that you apply to the dataset group to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>Maximum number of tags per resource - 50.</p> </li>
    /// <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>
    /// <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li>
    /// <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li>
    /// <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>
    /// <li> <p>Tag keys and values are case sensitive.</p> </li>
    /// <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li>
    /// </ul>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The optional metadata that you apply to the dataset group to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define.</p>
    /// <p>The following basic restrictions apply to tags:</p>
    /// <ul>
    /// <li> <p>Maximum number of tags per resource - 50.</p> </li>
    /// <li> <p>For each resource, each tag key must be unique, and each tag key can have only one value.</p> </li>
    /// <li> <p>Maximum key length - 128 Unicode characters in UTF-8.</p> </li>
    /// <li> <p>Maximum value length - 256 Unicode characters in UTF-8.</p> </li>
    /// <li> <p>If your tagging schema is used across multiple services and resources, remember that other services may have restrictions on allowed characters. Generally allowed characters are: letters, numbers, and spaces representable in UTF-8, and the following characters: + - = . _ : / @.</p> </li>
    /// <li> <p>Tag keys and values are case sensitive.</p> </li>
    /// <li> <p>Do not use <code>aws:</code>, <code>AWS:</code>, or any upper or lowercase combination of such as a prefix for keys as it is reserved for Amazon Web Services use. You cannot edit or delete tag keys with this prefix. Values can have this prefix. If a tag value has <code>aws</code> as its prefix but the key does not, then Forecast considers it to be a user tag and will count against the limit of 50 tags. Tags with only the key prefix of <code>aws</code> do not count against your tags per resource limit.</p> </li>
    /// </ul>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
