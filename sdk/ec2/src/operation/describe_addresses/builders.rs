// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_addresses::_describe_addresses_output::DescribeAddressesOutputBuilder;

pub use crate::operation::describe_addresses::_describe_addresses_input::DescribeAddressesInputBuilder;

/// Fluent builder constructing a request to `DescribeAddresses`.
///
/// <p>Describes the specified Elastic IP addresses or all of your Elastic IP addresses.</p>
/// <p>An Elastic IP address is for use in either the EC2-Classic platform or in a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html">Elastic IP Addresses</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p> <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAddressesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_addresses::builders::DescribeAddressesInputBuilder,
}
impl DescribeAddressesFluentBuilder {
    /// Creates a new `DescribeAddresses`.
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
            crate::operation::describe_addresses::DescribeAddresses,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_addresses::DescribeAddressesError,
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
        crate::operation::describe_addresses::DescribeAddressesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_addresses::DescribeAddressesError,
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
        crate::operation::describe_addresses::DescribeAddressesOutput,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_addresses::DescribeAddressesError,
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
            crate::operation::describe_addresses::DescribeAddresses,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<
            crate::operation::describe_addresses::DescribeAddressesError,
        >,
    > {
        self.customize_middleware().await
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li> <p> <code>allocation-id</code> - [EC2-VPC] The allocation ID for the address.</p> </li>
    /// <li> <p> <code>association-id</code> - [EC2-VPC] The association ID for the address.</p> </li>
    /// <li> <p> <code>domain</code> - Indicates whether the address is for use in EC2-Classic (<code>standard</code>) or in a VPC (<code>vpc</code>).</p> </li>
    /// <li> <p> <code>instance-id</code> - The ID of the instance the address is associated with, if any.</p> </li>
    /// <li> <p> <code>network-border-group</code> - A unique set of Availability Zones, Local Zones, or Wavelength Zones from where Amazon Web Services advertises IP addresses. </p> </li>
    /// <li> <p> <code>network-interface-id</code> - [EC2-VPC] The ID of the network interface that the address is associated with, if any.</p> </li>
    /// <li> <p> <code>network-interface-owner-id</code> - The Amazon Web Services account ID of the owner.</p> </li>
    /// <li> <p> <code>private-ip-address</code> - [EC2-VPC] The private IP address associated with the Elastic IP address.</p> </li>
    /// <li> <p> <code>public-ip</code> - The Elastic IP address, or the carrier IP address.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li> <p> <code>allocation-id</code> - [EC2-VPC] The allocation ID for the address.</p> </li>
    /// <li> <p> <code>association-id</code> - [EC2-VPC] The association ID for the address.</p> </li>
    /// <li> <p> <code>domain</code> - Indicates whether the address is for use in EC2-Classic (<code>standard</code>) or in a VPC (<code>vpc</code>).</p> </li>
    /// <li> <p> <code>instance-id</code> - The ID of the instance the address is associated with, if any.</p> </li>
    /// <li> <p> <code>network-border-group</code> - A unique set of Availability Zones, Local Zones, or Wavelength Zones from where Amazon Web Services advertises IP addresses. </p> </li>
    /// <li> <p> <code>network-interface-id</code> - [EC2-VPC] The ID of the network interface that the address is associated with, if any.</p> </li>
    /// <li> <p> <code>network-interface-owner-id</code> - The Amazon Web Services account ID of the owner.</p> </li>
    /// <li> <p> <code>private-ip-address</code> - [EC2-VPC] The private IP address associated with the Elastic IP address.</p> </li>
    /// <li> <p> <code>public-ip</code> - The Elastic IP address, or the carrier IP address.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// Appends an item to `PublicIps`.
    ///
    /// To override the contents of this collection use [`set_public_ips`](Self::set_public_ips).
    ///
    /// <p>One or more Elastic IP addresses.</p>
    /// <p>Default: Describes all your Elastic IP addresses.</p>
    pub fn public_ips(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.public_ips(input.into());
        self
    }
    /// <p>One or more Elastic IP addresses.</p>
    /// <p>Default: Describes all your Elastic IP addresses.</p>
    pub fn set_public_ips(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_public_ips(input);
        self
    }
    /// Appends an item to `AllocationIds`.
    ///
    /// To override the contents of this collection use [`set_allocation_ids`](Self::set_allocation_ids).
    ///
    /// <p>[EC2-VPC] Information about the allocation IDs.</p>
    pub fn allocation_ids(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.allocation_ids(input.into());
        self
    }
    /// <p>[EC2-VPC] Information about the allocation IDs.</p>
    pub fn set_allocation_ids(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_allocation_ids(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
