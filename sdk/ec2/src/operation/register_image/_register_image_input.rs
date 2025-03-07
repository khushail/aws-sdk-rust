// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for RegisterImage.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterImageInput {
    /// <p>The full path to your AMI manifest in Amazon S3 storage. The specified bucket must have the <code>aws-exec-read</code> canned access control list (ACL) to ensure that it can be accessed by Amazon EC2. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl">Canned ACLs</a> in the <i>Amazon S3 Service Developer Guide</i>.</p>
    #[doc(hidden)]
    pub image_location: ::std::option::Option<::std::string::String>,
    /// <p>The architecture of the AMI.</p>
    /// <p>Default: For Amazon EBS-backed AMIs, <code>i386</code>. For instance store-backed AMIs, the architecture specified in the manifest file.</p>
    #[doc(hidden)]
    pub architecture: ::std::option::Option<crate::types::ArchitectureValues>,
    /// <p>The block device mapping entries.</p>
    /// <p>If you specify an Amazon EBS volume using the ID of an Amazon EBS snapshot, you can't specify the encryption state of the volume.</p>
    /// <p>If you create an AMI on an Outpost, then all backing snapshots must be on the same Outpost or in the Region of that Outpost. AMIs on an Outpost that include local snapshots can be used to launch instances on the same Outpost only. For more information, <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#ami">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub block_device_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::BlockDeviceMapping>>,
    /// <p>A description for your AMI.</p>
    #[doc(hidden)]
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Set to <code>true</code> to enable enhanced networking with ENA for the AMI and any instances that you launch from the AMI.</p>
    /// <p>This option is supported only for HVM AMIs. Specifying this option with a PV AMI can make instances launched from the AMI unreachable.</p>
    #[doc(hidden)]
    pub ena_support: ::std::option::Option<bool>,
    /// <p>The ID of the kernel.</p>
    #[doc(hidden)]
    pub kernel_id: ::std::option::Option<::std::string::String>,
    /// <p>A name for your AMI.</p>
    /// <p>Constraints: 3-128 alphanumeric characters, parentheses (()), square brackets ([]), spaces ( ), periods (.), slashes (/), dashes (-), single quotes ('), at-signs (@), or underscores(_)</p>
    #[doc(hidden)]
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The billing product codes. Your account must be authorized to specify billing product codes.</p>
    /// <p>If your account is not authorized to specify billing product codes, you can publish AMIs that include billable software and list them on the Amazon Web Services Marketplace. You must first register as a seller on the Amazon Web Services Marketplace. For more information, see <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/user-guide-for-sellers.html">Getting started as a seller</a> and <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/ami-products.html">AMI-based products</a> in the <i>Amazon Web Services Marketplace Seller Guide</i>.</p>
    #[doc(hidden)]
    pub billing_products: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The ID of the RAM disk.</p>
    #[doc(hidden)]
    pub ramdisk_id: ::std::option::Option<::std::string::String>,
    /// <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    #[doc(hidden)]
    pub root_device_name: ::std::option::Option<::std::string::String>,
    /// <p>Set to <code>simple</code> to enable enhanced networking with the Intel 82599 Virtual Function interface for the AMI and any instances that you launch from the AMI.</p>
    /// <p>There is no way to disable <code>sriovNetSupport</code> at this time.</p>
    /// <p>This option is supported only for HVM AMIs. Specifying this option with a PV AMI can make instances launched from the AMI unreachable.</p>
    #[doc(hidden)]
    pub sriov_net_support: ::std::option::Option<::std::string::String>,
    /// <p>The type of virtualization (<code>hvm</code> | <code>paravirtual</code>).</p>
    /// <p>Default: <code>paravirtual</code> </p>
    #[doc(hidden)]
    pub virtualization_type: ::std::option::Option<::std::string::String>,
    /// <p>The boot mode of the AMI. A value of <code>uefi-preferred</code> indicates that the AMI supports both UEFI and Legacy BIOS.</p> <note>
    /// <p>The operating system contained in the AMI must be configured to support the specified boot mode.</p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html">Boot modes</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub boot_mode: ::std::option::Option<crate::types::BootModeValues>,
    /// <p>Set to <code>v2.0</code> to enable Trusted Platform Module (TPM) support. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html">NitroTPM</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub tpm_support: ::std::option::Option<crate::types::TpmSupportValues>,
    /// <p>Base64 representation of the non-volatile UEFI variable store. To retrieve the UEFI data, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetInstanceUefiData">GetInstanceUefiData</a> command. You can inspect and modify the UEFI data by using the <a href="https://github.com/awslabs/python-uefivars">python-uefivars tool</a> on GitHub. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/uefi-secure-boot.html">UEFI Secure Boot</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub uefi_data: ::std::option::Option<::std::string::String>,
    /// <p>Set to <code>v2.0</code> to indicate that IMDSv2 is specified in the AMI. Instances launched from this AMI will have <code>HttpTokens</code> automatically set to <code>required</code> so that, by default, the instance requires that IMDSv2 is used when requesting instance metadata. In addition, <code>HttpPutResponseHopLimit</code> is set to <code>2</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration">Configure the AMI</a> in the <i>Amazon EC2 User Guide</i>.</p> <note>
    /// <p>If you set the value to <code>v2.0</code>, make sure that your AMI software can support IMDSv2.</p>
    /// </note>
    #[doc(hidden)]
    pub imds_support: ::std::option::Option<crate::types::ImdsSupportValues>,
}
impl RegisterImageInput {
    /// <p>The full path to your AMI manifest in Amazon S3 storage. The specified bucket must have the <code>aws-exec-read</code> canned access control list (ACL) to ensure that it can be accessed by Amazon EC2. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl">Canned ACLs</a> in the <i>Amazon S3 Service Developer Guide</i>.</p>
    pub fn image_location(&self) -> ::std::option::Option<&str> {
        self.image_location.as_deref()
    }
    /// <p>The architecture of the AMI.</p>
    /// <p>Default: For Amazon EBS-backed AMIs, <code>i386</code>. For instance store-backed AMIs, the architecture specified in the manifest file.</p>
    pub fn architecture(&self) -> ::std::option::Option<&crate::types::ArchitectureValues> {
        self.architecture.as_ref()
    }
    /// <p>The block device mapping entries.</p>
    /// <p>If you specify an Amazon EBS volume using the ID of an Amazon EBS snapshot, you can't specify the encryption state of the volume.</p>
    /// <p>If you create an AMI on an Outpost, then all backing snapshots must be on the same Outpost or in the Region of that Outpost. AMIs on an Outpost that include local snapshots can be used to launch instances on the same Outpost only. For more information, <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#ami">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn block_device_mappings(
        &self,
    ) -> ::std::option::Option<&[crate::types::BlockDeviceMapping]> {
        self.block_device_mappings.as_deref()
    }
    /// <p>A description for your AMI.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Set to <code>true</code> to enable enhanced networking with ENA for the AMI and any instances that you launch from the AMI.</p>
    /// <p>This option is supported only for HVM AMIs. Specifying this option with a PV AMI can make instances launched from the AMI unreachable.</p>
    pub fn ena_support(&self) -> ::std::option::Option<bool> {
        self.ena_support
    }
    /// <p>The ID of the kernel.</p>
    pub fn kernel_id(&self) -> ::std::option::Option<&str> {
        self.kernel_id.as_deref()
    }
    /// <p>A name for your AMI.</p>
    /// <p>Constraints: 3-128 alphanumeric characters, parentheses (()), square brackets ([]), spaces ( ), periods (.), slashes (/), dashes (-), single quotes ('), at-signs (@), or underscores(_)</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The billing product codes. Your account must be authorized to specify billing product codes.</p>
    /// <p>If your account is not authorized to specify billing product codes, you can publish AMIs that include billable software and list them on the Amazon Web Services Marketplace. You must first register as a seller on the Amazon Web Services Marketplace. For more information, see <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/user-guide-for-sellers.html">Getting started as a seller</a> and <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/ami-products.html">AMI-based products</a> in the <i>Amazon Web Services Marketplace Seller Guide</i>.</p>
    pub fn billing_products(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.billing_products.as_deref()
    }
    /// <p>The ID of the RAM disk.</p>
    pub fn ramdisk_id(&self) -> ::std::option::Option<&str> {
        self.ramdisk_id.as_deref()
    }
    /// <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    pub fn root_device_name(&self) -> ::std::option::Option<&str> {
        self.root_device_name.as_deref()
    }
    /// <p>Set to <code>simple</code> to enable enhanced networking with the Intel 82599 Virtual Function interface for the AMI and any instances that you launch from the AMI.</p>
    /// <p>There is no way to disable <code>sriovNetSupport</code> at this time.</p>
    /// <p>This option is supported only for HVM AMIs. Specifying this option with a PV AMI can make instances launched from the AMI unreachable.</p>
    pub fn sriov_net_support(&self) -> ::std::option::Option<&str> {
        self.sriov_net_support.as_deref()
    }
    /// <p>The type of virtualization (<code>hvm</code> | <code>paravirtual</code>).</p>
    /// <p>Default: <code>paravirtual</code> </p>
    pub fn virtualization_type(&self) -> ::std::option::Option<&str> {
        self.virtualization_type.as_deref()
    }
    /// <p>The boot mode of the AMI. A value of <code>uefi-preferred</code> indicates that the AMI supports both UEFI and Legacy BIOS.</p> <note>
    /// <p>The operating system contained in the AMI must be configured to support the specified boot mode.</p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html">Boot modes</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn boot_mode(&self) -> ::std::option::Option<&crate::types::BootModeValues> {
        self.boot_mode.as_ref()
    }
    /// <p>Set to <code>v2.0</code> to enable Trusted Platform Module (TPM) support. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html">NitroTPM</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn tpm_support(&self) -> ::std::option::Option<&crate::types::TpmSupportValues> {
        self.tpm_support.as_ref()
    }
    /// <p>Base64 representation of the non-volatile UEFI variable store. To retrieve the UEFI data, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetInstanceUefiData">GetInstanceUefiData</a> command. You can inspect and modify the UEFI data by using the <a href="https://github.com/awslabs/python-uefivars">python-uefivars tool</a> on GitHub. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/uefi-secure-boot.html">UEFI Secure Boot</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn uefi_data(&self) -> ::std::option::Option<&str> {
        self.uefi_data.as_deref()
    }
    /// <p>Set to <code>v2.0</code> to indicate that IMDSv2 is specified in the AMI. Instances launched from this AMI will have <code>HttpTokens</code> automatically set to <code>required</code> so that, by default, the instance requires that IMDSv2 is used when requesting instance metadata. In addition, <code>HttpPutResponseHopLimit</code> is set to <code>2</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration">Configure the AMI</a> in the <i>Amazon EC2 User Guide</i>.</p> <note>
    /// <p>If you set the value to <code>v2.0</code>, make sure that your AMI software can support IMDSv2.</p>
    /// </note>
    pub fn imds_support(&self) -> ::std::option::Option<&crate::types::ImdsSupportValues> {
        self.imds_support.as_ref()
    }
}
impl RegisterImageInput {
    /// Creates a new builder-style object to manufacture [`RegisterImageInput`](crate::operation::register_image::RegisterImageInput).
    pub fn builder() -> crate::operation::register_image::builders::RegisterImageInputBuilder {
        crate::operation::register_image::builders::RegisterImageInputBuilder::default()
    }
}

/// A builder for [`RegisterImageInput`](crate::operation::register_image::RegisterImageInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct RegisterImageInputBuilder {
    pub(crate) image_location: ::std::option::Option<::std::string::String>,
    pub(crate) architecture: ::std::option::Option<crate::types::ArchitectureValues>,
    pub(crate) block_device_mappings:
        ::std::option::Option<::std::vec::Vec<crate::types::BlockDeviceMapping>>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ena_support: ::std::option::Option<bool>,
    pub(crate) kernel_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) billing_products: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) ramdisk_id: ::std::option::Option<::std::string::String>,
    pub(crate) root_device_name: ::std::option::Option<::std::string::String>,
    pub(crate) sriov_net_support: ::std::option::Option<::std::string::String>,
    pub(crate) virtualization_type: ::std::option::Option<::std::string::String>,
    pub(crate) boot_mode: ::std::option::Option<crate::types::BootModeValues>,
    pub(crate) tpm_support: ::std::option::Option<crate::types::TpmSupportValues>,
    pub(crate) uefi_data: ::std::option::Option<::std::string::String>,
    pub(crate) imds_support: ::std::option::Option<crate::types::ImdsSupportValues>,
}
impl RegisterImageInputBuilder {
    /// <p>The full path to your AMI manifest in Amazon S3 storage. The specified bucket must have the <code>aws-exec-read</code> canned access control list (ACL) to ensure that it can be accessed by Amazon EC2. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl">Canned ACLs</a> in the <i>Amazon S3 Service Developer Guide</i>.</p>
    pub fn image_location(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.image_location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The full path to your AMI manifest in Amazon S3 storage. The specified bucket must have the <code>aws-exec-read</code> canned access control list (ACL) to ensure that it can be accessed by Amazon EC2. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/acl-overview.html#canned-acl">Canned ACLs</a> in the <i>Amazon S3 Service Developer Guide</i>.</p>
    pub fn set_image_location(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.image_location = input;
        self
    }
    /// <p>The architecture of the AMI.</p>
    /// <p>Default: For Amazon EBS-backed AMIs, <code>i386</code>. For instance store-backed AMIs, the architecture specified in the manifest file.</p>
    pub fn architecture(mut self, input: crate::types::ArchitectureValues) -> Self {
        self.architecture = ::std::option::Option::Some(input);
        self
    }
    /// <p>The architecture of the AMI.</p>
    /// <p>Default: For Amazon EBS-backed AMIs, <code>i386</code>. For instance store-backed AMIs, the architecture specified in the manifest file.</p>
    pub fn set_architecture(
        mut self,
        input: ::std::option::Option<crate::types::ArchitectureValues>,
    ) -> Self {
        self.architecture = input;
        self
    }
    /// Appends an item to `block_device_mappings`.
    ///
    /// To override the contents of this collection use [`set_block_device_mappings`](Self::set_block_device_mappings).
    ///
    /// <p>The block device mapping entries.</p>
    /// <p>If you specify an Amazon EBS volume using the ID of an Amazon EBS snapshot, you can't specify the encryption state of the volume.</p>
    /// <p>If you create an AMI on an Outpost, then all backing snapshots must be on the same Outpost or in the Region of that Outpost. AMIs on an Outpost that include local snapshots can be used to launch instances on the same Outpost only. For more information, <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#ami">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn block_device_mappings(mut self, input: crate::types::BlockDeviceMapping) -> Self {
        let mut v = self.block_device_mappings.unwrap_or_default();
        v.push(input);
        self.block_device_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>The block device mapping entries.</p>
    /// <p>If you specify an Amazon EBS volume using the ID of an Amazon EBS snapshot, you can't specify the encryption state of the volume.</p>
    /// <p>If you create an AMI on an Outpost, then all backing snapshots must be on the same Outpost or in the Region of that Outpost. AMIs on an Outpost that include local snapshots can be used to launch instances on the same Outpost only. For more information, <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#ami">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_block_device_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BlockDeviceMapping>>,
    ) -> Self {
        self.block_device_mappings = input;
        self
    }
    /// <p>A description for your AMI.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for your AMI.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Set to <code>true</code> to enable enhanced networking with ENA for the AMI and any instances that you launch from the AMI.</p>
    /// <p>This option is supported only for HVM AMIs. Specifying this option with a PV AMI can make instances launched from the AMI unreachable.</p>
    pub fn ena_support(mut self, input: bool) -> Self {
        self.ena_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set to <code>true</code> to enable enhanced networking with ENA for the AMI and any instances that you launch from the AMI.</p>
    /// <p>This option is supported only for HVM AMIs. Specifying this option with a PV AMI can make instances launched from the AMI unreachable.</p>
    pub fn set_ena_support(mut self, input: ::std::option::Option<bool>) -> Self {
        self.ena_support = input;
        self
    }
    /// <p>The ID of the kernel.</p>
    pub fn kernel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kernel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the kernel.</p>
    pub fn set_kernel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kernel_id = input;
        self
    }
    /// <p>A name for your AMI.</p>
    /// <p>Constraints: 3-128 alphanumeric characters, parentheses (()), square brackets ([]), spaces ( ), periods (.), slashes (/), dashes (-), single quotes ('), at-signs (@), or underscores(_)</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name for your AMI.</p>
    /// <p>Constraints: 3-128 alphanumeric characters, parentheses (()), square brackets ([]), spaces ( ), periods (.), slashes (/), dashes (-), single quotes ('), at-signs (@), or underscores(_)</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// Appends an item to `billing_products`.
    ///
    /// To override the contents of this collection use [`set_billing_products`](Self::set_billing_products).
    ///
    /// <p>The billing product codes. Your account must be authorized to specify billing product codes.</p>
    /// <p>If your account is not authorized to specify billing product codes, you can publish AMIs that include billable software and list them on the Amazon Web Services Marketplace. You must first register as a seller on the Amazon Web Services Marketplace. For more information, see <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/user-guide-for-sellers.html">Getting started as a seller</a> and <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/ami-products.html">AMI-based products</a> in the <i>Amazon Web Services Marketplace Seller Guide</i>.</p>
    pub fn billing_products(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.billing_products.unwrap_or_default();
        v.push(input.into());
        self.billing_products = ::std::option::Option::Some(v);
        self
    }
    /// <p>The billing product codes. Your account must be authorized to specify billing product codes.</p>
    /// <p>If your account is not authorized to specify billing product codes, you can publish AMIs that include billable software and list them on the Amazon Web Services Marketplace. You must first register as a seller on the Amazon Web Services Marketplace. For more information, see <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/user-guide-for-sellers.html">Getting started as a seller</a> and <a href="https://docs.aws.amazon.com/marketplace/latest/userguide/ami-products.html">AMI-based products</a> in the <i>Amazon Web Services Marketplace Seller Guide</i>.</p>
    pub fn set_billing_products(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.billing_products = input;
        self
    }
    /// <p>The ID of the RAM disk.</p>
    pub fn ramdisk_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ramdisk_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the RAM disk.</p>
    pub fn set_ramdisk_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ramdisk_id = input;
        self
    }
    /// <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    pub fn root_device_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.root_device_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    pub fn set_root_device_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.root_device_name = input;
        self
    }
    /// <p>Set to <code>simple</code> to enable enhanced networking with the Intel 82599 Virtual Function interface for the AMI and any instances that you launch from the AMI.</p>
    /// <p>There is no way to disable <code>sriovNetSupport</code> at this time.</p>
    /// <p>This option is supported only for HVM AMIs. Specifying this option with a PV AMI can make instances launched from the AMI unreachable.</p>
    pub fn sriov_net_support(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.sriov_net_support = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Set to <code>simple</code> to enable enhanced networking with the Intel 82599 Virtual Function interface for the AMI and any instances that you launch from the AMI.</p>
    /// <p>There is no way to disable <code>sriovNetSupport</code> at this time.</p>
    /// <p>This option is supported only for HVM AMIs. Specifying this option with a PV AMI can make instances launched from the AMI unreachable.</p>
    pub fn set_sriov_net_support(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.sriov_net_support = input;
        self
    }
    /// <p>The type of virtualization (<code>hvm</code> | <code>paravirtual</code>).</p>
    /// <p>Default: <code>paravirtual</code> </p>
    pub fn virtualization_type(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.virtualization_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of virtualization (<code>hvm</code> | <code>paravirtual</code>).</p>
    /// <p>Default: <code>paravirtual</code> </p>
    pub fn set_virtualization_type(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.virtualization_type = input;
        self
    }
    /// <p>The boot mode of the AMI. A value of <code>uefi-preferred</code> indicates that the AMI supports both UEFI and Legacy BIOS.</p> <note>
    /// <p>The operating system contained in the AMI must be configured to support the specified boot mode.</p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html">Boot modes</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn boot_mode(mut self, input: crate::types::BootModeValues) -> Self {
        self.boot_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The boot mode of the AMI. A value of <code>uefi-preferred</code> indicates that the AMI supports both UEFI and Legacy BIOS.</p> <note>
    /// <p>The operating system contained in the AMI must be configured to support the specified boot mode.</p>
    /// </note>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ami-boot.html">Boot modes</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_boot_mode(
        mut self,
        input: ::std::option::Option<crate::types::BootModeValues>,
    ) -> Self {
        self.boot_mode = input;
        self
    }
    /// <p>Set to <code>v2.0</code> to enable Trusted Platform Module (TPM) support. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html">NitroTPM</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn tpm_support(mut self, input: crate::types::TpmSupportValues) -> Self {
        self.tpm_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set to <code>v2.0</code> to enable Trusted Platform Module (TPM) support. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/nitrotpm.html">NitroTPM</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_tpm_support(
        mut self,
        input: ::std::option::Option<crate::types::TpmSupportValues>,
    ) -> Self {
        self.tpm_support = input;
        self
    }
    /// <p>Base64 representation of the non-volatile UEFI variable store. To retrieve the UEFI data, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetInstanceUefiData">GetInstanceUefiData</a> command. You can inspect and modify the UEFI data by using the <a href="https://github.com/awslabs/python-uefivars">python-uefivars tool</a> on GitHub. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/uefi-secure-boot.html">UEFI Secure Boot</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn uefi_data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.uefi_data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Base64 representation of the non-volatile UEFI variable store. To retrieve the UEFI data, use the <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetInstanceUefiData">GetInstanceUefiData</a> command. You can inspect and modify the UEFI data by using the <a href="https://github.com/awslabs/python-uefivars">python-uefivars tool</a> on GitHub. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/uefi-secure-boot.html">UEFI Secure Boot</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_uefi_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.uefi_data = input;
        self
    }
    /// <p>Set to <code>v2.0</code> to indicate that IMDSv2 is specified in the AMI. Instances launched from this AMI will have <code>HttpTokens</code> automatically set to <code>required</code> so that, by default, the instance requires that IMDSv2 is used when requesting instance metadata. In addition, <code>HttpPutResponseHopLimit</code> is set to <code>2</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration">Configure the AMI</a> in the <i>Amazon EC2 User Guide</i>.</p> <note>
    /// <p>If you set the value to <code>v2.0</code>, make sure that your AMI software can support IMDSv2.</p>
    /// </note>
    pub fn imds_support(mut self, input: crate::types::ImdsSupportValues) -> Self {
        self.imds_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set to <code>v2.0</code> to indicate that IMDSv2 is specified in the AMI. Instances launched from this AMI will have <code>HttpTokens</code> automatically set to <code>required</code> so that, by default, the instance requires that IMDSv2 is used when requesting instance metadata. In addition, <code>HttpPutResponseHopLimit</code> is set to <code>2</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/configuring-IMDS-new-instances.html#configure-IMDS-new-instances-ami-configuration">Configure the AMI</a> in the <i>Amazon EC2 User Guide</i>.</p> <note>
    /// <p>If you set the value to <code>v2.0</code>, make sure that your AMI software can support IMDSv2.</p>
    /// </note>
    pub fn set_imds_support(
        mut self,
        input: ::std::option::Option<crate::types::ImdsSupportValues>,
    ) -> Self {
        self.imds_support = input;
        self
    }
    /// Consumes the builder and constructs a [`RegisterImageInput`](crate::operation::register_image::RegisterImageInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::register_image::RegisterImageInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::register_image::RegisterImageInput {
            image_location: self.image_location,
            architecture: self.architecture,
            block_device_mappings: self.block_device_mappings,
            description: self.description,
            dry_run: self.dry_run,
            ena_support: self.ena_support,
            kernel_id: self.kernel_id,
            name: self.name,
            billing_products: self.billing_products,
            ramdisk_id: self.ramdisk_id,
            root_device_name: self.root_device_name,
            sriov_net_support: self.sriov_net_support,
            virtualization_type: self.virtualization_type,
            boot_mode: self.boot_mode,
            tpm_support: self.tpm_support,
            uefi_data: self.uefi_data,
            imds_support: self.imds_support,
        })
    }
}
