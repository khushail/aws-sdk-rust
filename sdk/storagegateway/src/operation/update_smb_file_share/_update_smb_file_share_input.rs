// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>UpdateSMBFileShareInput</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateSmbFileShareInput {
    /// <p>The Amazon Resource Name (ARN) of the SMB file share that you want to update.</p>
    #[doc(hidden)]
    pub file_share_arn: ::std::option::Option<::std::string::String>,
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[doc(hidden)]
    pub kms_encrypted: ::std::option::Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    #[doc(hidden)]
    pub kms_key: ::std::option::Option<::std::string::String>,
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the S3 File Gateway. The default value is <code>S3_STANDARD</code>. Optional.</p>
    /// <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    #[doc(hidden)]
    pub default_storage_class: ::std::option::Option<::std::string::String>,
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a S3 File Gateway puts objects into. The default value is <code>private</code>.</p>
    #[doc(hidden)]
    pub object_acl: ::std::option::Option<crate::types::ObjectAcl>,
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set write status to read-only, otherwise set to <code>false</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[doc(hidden)]
    pub read_only: ::std::option::Option<bool>,
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[doc(hidden)]
    pub guess_mime_type_enabled: ::std::option::Option<bool>,
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note>
    /// <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[doc(hidden)]
    pub requester_pays: ::std::option::Option<bool>,
    /// <p>Set this value to <code>true</code> to enable access control list (ACL) on the SMB file share. Set it to <code>false</code> to map file and directory permissions to the POSIX permissions.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/smb-acl.html">Using Microsoft Windows ACLs to control access to an SMB file share</a> in the <i>Storage Gateway User Guide</i>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[doc(hidden)]
    pub smbacl_enabled: ::std::option::Option<bool>,
    /// <p>The files and folders on this share will only be visible to users with read access.</p>
    #[doc(hidden)]
    pub access_based_enumeration: ::std::option::Option<bool>,
    /// <p>A list of users or groups in the Active Directory that have administrator rights to the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[doc(hidden)]
    pub admin_user_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of users or groups in the Active Directory that are allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[doc(hidden)]
    pub valid_user_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A list of users or groups in the Active Directory that are not allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    #[doc(hidden)]
    pub invalid_user_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    #[doc(hidden)]
    pub audit_destination_arn: ::std::option::Option<::std::string::String>,
    /// <p>The case of an object name in an Amazon S3 bucket. For <code>ClientSpecified</code>, the client determines the case sensitivity. For <code>CaseSensitive</code>, the gateway determines the case sensitivity. The default value is <code>ClientSpecified</code>.</p>
    #[doc(hidden)]
    pub case_sensitivity: ::std::option::Option<crate::types::CaseSensitivity>,
    /// <p>The name of the file share. Optional.</p> <note>
    /// <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>, or if an access point or access point alias is used.</p>
    /// </note>
    #[doc(hidden)]
    pub file_share_name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies refresh cache information for the file share.</p>
    #[doc(hidden)]
    pub cache_attributes: ::std::option::Option<crate::types::CacheAttributes>,
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note>
    /// <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p>
    /// </note>
    /// <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p>
    /// <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p>
    /// <p>The following example sets <code>NotificationPolicy</code> off.</p>
    /// <p> <code>{}</code> </p>
    #[doc(hidden)]
    pub notification_policy: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether opportunistic locking is enabled for the SMB file share.</p> <note>
    /// <p>Enabling opportunistic locking on case-sensitive shares is not recommended for workloads that involve access to files with the same name in different case.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    #[doc(hidden)]
    pub oplocks_enabled: ::std::option::Option<bool>,
}
impl UpdateSmbFileShareInput {
    /// <p>The Amazon Resource Name (ARN) of the SMB file share that you want to update.</p>
    pub fn file_share_arn(&self) -> ::std::option::Option<&str> {
        self.file_share_arn.as_deref()
    }
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn kms_encrypted(&self) -> ::std::option::Option<bool> {
        self.kms_encrypted
    }
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    pub fn kms_key(&self) -> ::std::option::Option<&str> {
        self.kms_key.as_deref()
    }
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the S3 File Gateway. The default value is <code>S3_STANDARD</code>. Optional.</p>
    /// <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    pub fn default_storage_class(&self) -> ::std::option::Option<&str> {
        self.default_storage_class.as_deref()
    }
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a S3 File Gateway puts objects into. The default value is <code>private</code>.</p>
    pub fn object_acl(&self) -> ::std::option::Option<&crate::types::ObjectAcl> {
        self.object_acl.as_ref()
    }
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set write status to read-only, otherwise set to <code>false</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn read_only(&self) -> ::std::option::Option<bool> {
        self.read_only
    }
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn guess_mime_type_enabled(&self) -> ::std::option::Option<bool> {
        self.guess_mime_type_enabled
    }
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note>
    /// <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn requester_pays(&self) -> ::std::option::Option<bool> {
        self.requester_pays
    }
    /// <p>Set this value to <code>true</code> to enable access control list (ACL) on the SMB file share. Set it to <code>false</code> to map file and directory permissions to the POSIX permissions.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/smb-acl.html">Using Microsoft Windows ACLs to control access to an SMB file share</a> in the <i>Storage Gateway User Guide</i>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn smbacl_enabled(&self) -> ::std::option::Option<bool> {
        self.smbacl_enabled
    }
    /// <p>The files and folders on this share will only be visible to users with read access.</p>
    pub fn access_based_enumeration(&self) -> ::std::option::Option<bool> {
        self.access_based_enumeration
    }
    /// <p>A list of users or groups in the Active Directory that have administrator rights to the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn admin_user_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.admin_user_list.as_deref()
    }
    /// <p>A list of users or groups in the Active Directory that are allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn valid_user_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.valid_user_list.as_deref()
    }
    /// <p>A list of users or groups in the Active Directory that are not allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn invalid_user_list(&self) -> ::std::option::Option<&[::std::string::String]> {
        self.invalid_user_list.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    pub fn audit_destination_arn(&self) -> ::std::option::Option<&str> {
        self.audit_destination_arn.as_deref()
    }
    /// <p>The case of an object name in an Amazon S3 bucket. For <code>ClientSpecified</code>, the client determines the case sensitivity. For <code>CaseSensitive</code>, the gateway determines the case sensitivity. The default value is <code>ClientSpecified</code>.</p>
    pub fn case_sensitivity(&self) -> ::std::option::Option<&crate::types::CaseSensitivity> {
        self.case_sensitivity.as_ref()
    }
    /// <p>The name of the file share. Optional.</p> <note>
    /// <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>, or if an access point or access point alias is used.</p>
    /// </note>
    pub fn file_share_name(&self) -> ::std::option::Option<&str> {
        self.file_share_name.as_deref()
    }
    /// <p>Specifies refresh cache information for the file share.</p>
    pub fn cache_attributes(&self) -> ::std::option::Option<&crate::types::CacheAttributes> {
        self.cache_attributes.as_ref()
    }
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note>
    /// <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p>
    /// </note>
    /// <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p>
    /// <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p>
    /// <p>The following example sets <code>NotificationPolicy</code> off.</p>
    /// <p> <code>{}</code> </p>
    pub fn notification_policy(&self) -> ::std::option::Option<&str> {
        self.notification_policy.as_deref()
    }
    /// <p>Specifies whether opportunistic locking is enabled for the SMB file share.</p> <note>
    /// <p>Enabling opportunistic locking on case-sensitive shares is not recommended for workloads that involve access to files with the same name in different case.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn oplocks_enabled(&self) -> ::std::option::Option<bool> {
        self.oplocks_enabled
    }
}
impl UpdateSmbFileShareInput {
    /// Creates a new builder-style object to manufacture [`UpdateSmbFileShareInput`](crate::operation::update_smb_file_share::UpdateSmbFileShareInput).
    pub fn builder(
    ) -> crate::operation::update_smb_file_share::builders::UpdateSmbFileShareInputBuilder {
        crate::operation::update_smb_file_share::builders::UpdateSmbFileShareInputBuilder::default()
    }
}

/// A builder for [`UpdateSmbFileShareInput`](crate::operation::update_smb_file_share::UpdateSmbFileShareInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct UpdateSmbFileShareInputBuilder {
    pub(crate) file_share_arn: ::std::option::Option<::std::string::String>,
    pub(crate) kms_encrypted: ::std::option::Option<bool>,
    pub(crate) kms_key: ::std::option::Option<::std::string::String>,
    pub(crate) default_storage_class: ::std::option::Option<::std::string::String>,
    pub(crate) object_acl: ::std::option::Option<crate::types::ObjectAcl>,
    pub(crate) read_only: ::std::option::Option<bool>,
    pub(crate) guess_mime_type_enabled: ::std::option::Option<bool>,
    pub(crate) requester_pays: ::std::option::Option<bool>,
    pub(crate) smbacl_enabled: ::std::option::Option<bool>,
    pub(crate) access_based_enumeration: ::std::option::Option<bool>,
    pub(crate) admin_user_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) valid_user_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) invalid_user_list: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) audit_destination_arn: ::std::option::Option<::std::string::String>,
    pub(crate) case_sensitivity: ::std::option::Option<crate::types::CaseSensitivity>,
    pub(crate) file_share_name: ::std::option::Option<::std::string::String>,
    pub(crate) cache_attributes: ::std::option::Option<crate::types::CacheAttributes>,
    pub(crate) notification_policy: ::std::option::Option<::std::string::String>,
    pub(crate) oplocks_enabled: ::std::option::Option<bool>,
}
impl UpdateSmbFileShareInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the SMB file share that you want to update.</p>
    pub fn file_share_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.file_share_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the SMB file share that you want to update.</p>
    pub fn set_file_share_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.file_share_arn = input;
        self
    }
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn kms_encrypted(mut self, input: bool) -> Self {
        self.kms_encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set to <code>true</code> to use Amazon S3 server-side encryption with your own KMS key, or <code>false</code> to use a key managed by Amazon S3. Optional.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_kms_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.kms_encrypted = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    pub fn kms_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of a symmetric customer master key (CMK) used for Amazon S3 server-side encryption. Storage Gateway does not support asymmetric CMKs. This value can only be set when <code>KMSEncrypted</code> is <code>true</code>. Optional.</p>
    pub fn set_kms_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key = input;
        self
    }
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the S3 File Gateway. The default value is <code>S3_STANDARD</code>. Optional.</p>
    /// <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    pub fn default_storage_class(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.default_storage_class = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The default storage class for objects put into an Amazon S3 bucket by the S3 File Gateway. The default value is <code>S3_STANDARD</code>. Optional.</p>
    /// <p>Valid Values: <code>S3_STANDARD</code> | <code>S3_INTELLIGENT_TIERING</code> | <code>S3_STANDARD_IA</code> | <code>S3_ONEZONE_IA</code> </p>
    pub fn set_default_storage_class(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.default_storage_class = input;
        self
    }
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a S3 File Gateway puts objects into. The default value is <code>private</code>.</p>
    pub fn object_acl(mut self, input: crate::types::ObjectAcl) -> Self {
        self.object_acl = ::std::option::Option::Some(input);
        self
    }
    /// <p>A value that sets the access control list (ACL) permission for objects in the S3 bucket that a S3 File Gateway puts objects into. The default value is <code>private</code>.</p>
    pub fn set_object_acl(mut self, input: ::std::option::Option<crate::types::ObjectAcl>) -> Self {
        self.object_acl = input;
        self
    }
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set write status to read-only, otherwise set to <code>false</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn read_only(mut self, input: bool) -> Self {
        self.read_only = ::std::option::Option::Some(input);
        self
    }
    /// <p>A value that sets the write status of a file share. Set this value to <code>true</code> to set write status to read-only, otherwise set to <code>false</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_read_only(mut self, input: ::std::option::Option<bool>) -> Self {
        self.read_only = input;
        self
    }
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn guess_mime_type_enabled(mut self, input: bool) -> Self {
        self.guess_mime_type_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>A value that enables guessing of the MIME type for uploaded objects based on file extensions. Set this value to <code>true</code> to enable MIME type guessing, otherwise set to <code>false</code>. The default value is <code>true</code>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_guess_mime_type_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.guess_mime_type_enabled = input;
        self
    }
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note>
    /// <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn requester_pays(mut self, input: bool) -> Self {
        self.requester_pays = ::std::option::Option::Some(input);
        self
    }
    /// <p>A value that sets who pays the cost of the request and the cost associated with data download from the S3 bucket. If this value is set to <code>true</code>, the requester pays the costs; otherwise, the S3 bucket owner pays. However, the S3 bucket owner always pays the cost of storing data.</p> <note>
    /// <p> <code>RequesterPays</code> is a configuration for the S3 bucket that backs the file share, so make sure that the configuration on the file share is the same as the S3 bucket configuration.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_requester_pays(mut self, input: ::std::option::Option<bool>) -> Self {
        self.requester_pays = input;
        self
    }
    /// <p>Set this value to <code>true</code> to enable access control list (ACL) on the SMB file share. Set it to <code>false</code> to map file and directory permissions to the POSIX permissions.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/smb-acl.html">Using Microsoft Windows ACLs to control access to an SMB file share</a> in the <i>Storage Gateway User Guide</i>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn smbacl_enabled(mut self, input: bool) -> Self {
        self.smbacl_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Set this value to <code>true</code> to enable access control list (ACL) on the SMB file share. Set it to <code>false</code> to map file and directory permissions to the POSIX permissions.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/storagegateway/latest/userguide/smb-acl.html">Using Microsoft Windows ACLs to control access to an SMB file share</a> in the <i>Storage Gateway User Guide</i>.</p>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_smbacl_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.smbacl_enabled = input;
        self
    }
    /// <p>The files and folders on this share will only be visible to users with read access.</p>
    pub fn access_based_enumeration(mut self, input: bool) -> Self {
        self.access_based_enumeration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The files and folders on this share will only be visible to users with read access.</p>
    pub fn set_access_based_enumeration(mut self, input: ::std::option::Option<bool>) -> Self {
        self.access_based_enumeration = input;
        self
    }
    /// Appends an item to `admin_user_list`.
    ///
    /// To override the contents of this collection use [`set_admin_user_list`](Self::set_admin_user_list).
    ///
    /// <p>A list of users or groups in the Active Directory that have administrator rights to the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn admin_user_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.admin_user_list.unwrap_or_default();
        v.push(input.into());
        self.admin_user_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of users or groups in the Active Directory that have administrator rights to the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn set_admin_user_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.admin_user_list = input;
        self
    }
    /// Appends an item to `valid_user_list`.
    ///
    /// To override the contents of this collection use [`set_valid_user_list`](Self::set_valid_user_list).
    ///
    /// <p>A list of users or groups in the Active Directory that are allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn valid_user_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.valid_user_list.unwrap_or_default();
        v.push(input.into());
        self.valid_user_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of users or groups in the Active Directory that are allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn set_valid_user_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.valid_user_list = input;
        self
    }
    /// Appends an item to `invalid_user_list`.
    ///
    /// To override the contents of this collection use [`set_invalid_user_list`](Self::set_invalid_user_list).
    ///
    /// <p>A list of users or groups in the Active Directory that are not allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn invalid_user_list(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut v = self.invalid_user_list.unwrap_or_default();
        v.push(input.into());
        self.invalid_user_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of users or groups in the Active Directory that are not allowed to access the file share. A group must be prefixed with the @ character. Acceptable formats include: <code>DOMAIN\User1</code>, <code>user1</code>, <code>@group1</code>, and <code>@DOMAIN\group1</code>. Can only be set if Authentication is set to <code>ActiveDirectory</code>.</p>
    pub fn set_invalid_user_list(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.invalid_user_list = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    pub fn audit_destination_arn(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.audit_destination_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the storage used for audit logs.</p>
    pub fn set_audit_destination_arn(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.audit_destination_arn = input;
        self
    }
    /// <p>The case of an object name in an Amazon S3 bucket. For <code>ClientSpecified</code>, the client determines the case sensitivity. For <code>CaseSensitive</code>, the gateway determines the case sensitivity. The default value is <code>ClientSpecified</code>.</p>
    pub fn case_sensitivity(mut self, input: crate::types::CaseSensitivity) -> Self {
        self.case_sensitivity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The case of an object name in an Amazon S3 bucket. For <code>ClientSpecified</code>, the client determines the case sensitivity. For <code>CaseSensitive</code>, the gateway determines the case sensitivity. The default value is <code>ClientSpecified</code>.</p>
    pub fn set_case_sensitivity(
        mut self,
        input: ::std::option::Option<crate::types::CaseSensitivity>,
    ) -> Self {
        self.case_sensitivity = input;
        self
    }
    /// <p>The name of the file share. Optional.</p> <note>
    /// <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>, or if an access point or access point alias is used.</p>
    /// </note>
    pub fn file_share_name(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.file_share_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the file share. Optional.</p> <note>
    /// <p> <code>FileShareName</code> must be set if an S3 prefix name is set in <code>LocationARN</code>, or if an access point or access point alias is used.</p>
    /// </note>
    pub fn set_file_share_name(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.file_share_name = input;
        self
    }
    /// <p>Specifies refresh cache information for the file share.</p>
    pub fn cache_attributes(mut self, input: crate::types::CacheAttributes) -> Self {
        self.cache_attributes = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies refresh cache information for the file share.</p>
    pub fn set_cache_attributes(
        mut self,
        input: ::std::option::Option<crate::types::CacheAttributes>,
    ) -> Self {
        self.cache_attributes = input;
        self
    }
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note>
    /// <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p>
    /// </note>
    /// <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p>
    /// <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p>
    /// <p>The following example sets <code>NotificationPolicy</code> off.</p>
    /// <p> <code>{}</code> </p>
    pub fn notification_policy(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.notification_policy = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The notification policy of the file share. <code>SettlingTimeInSeconds</code> controls the number of seconds to wait after the last point in time a client wrote to a file before generating an <code>ObjectUploaded</code> notification. Because clients can make many small writes to files, it's best to set this parameter for as long as possible to avoid generating multiple notifications for the same file in a small time period.</p> <note>
    /// <p> <code>SettlingTimeInSeconds</code> has no effect on the timing of the object uploading to Amazon S3, only the timing of the notification.</p>
    /// </note>
    /// <p>The following example sets <code>NotificationPolicy</code> on with <code>SettlingTimeInSeconds</code> set to 60.</p>
    /// <p> <code>{\"Upload\": {\"SettlingTimeInSeconds\": 60}}</code> </p>
    /// <p>The following example sets <code>NotificationPolicy</code> off.</p>
    /// <p> <code>{}</code> </p>
    pub fn set_notification_policy(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.notification_policy = input;
        self
    }
    /// <p>Specifies whether opportunistic locking is enabled for the SMB file share.</p> <note>
    /// <p>Enabling opportunistic locking on case-sensitive shares is not recommended for workloads that involve access to files with the same name in different case.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn oplocks_enabled(mut self, input: bool) -> Self {
        self.oplocks_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether opportunistic locking is enabled for the SMB file share.</p> <note>
    /// <p>Enabling opportunistic locking on case-sensitive shares is not recommended for workloads that involve access to files with the same name in different case.</p>
    /// </note>
    /// <p>Valid Values: <code>true</code> | <code>false</code> </p>
    pub fn set_oplocks_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.oplocks_enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`UpdateSmbFileShareInput`](crate::operation::update_smb_file_share::UpdateSmbFileShareInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_smb_file_share::UpdateSmbFileShareInput,
        ::aws_smithy_http::operation::error::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::update_smb_file_share::UpdateSmbFileShareInput {
                file_share_arn: self.file_share_arn,
                kms_encrypted: self.kms_encrypted,
                kms_key: self.kms_key,
                default_storage_class: self.default_storage_class,
                object_acl: self.object_acl,
                read_only: self.read_only,
                guess_mime_type_enabled: self.guess_mime_type_enabled,
                requester_pays: self.requester_pays,
                smbacl_enabled: self.smbacl_enabled,
                access_based_enumeration: self.access_based_enumeration,
                admin_user_list: self.admin_user_list,
                valid_user_list: self.valid_user_list,
                invalid_user_list: self.invalid_user_list,
                audit_destination_arn: self.audit_destination_arn,
                case_sensitivity: self.case_sensitivity,
                file_share_name: self.file_share_name,
                cache_attributes: self.cache_attributes,
                notification_policy: self.notification_policy,
                oplocks_enabled: self.oplocks_enabled,
            },
        )
    }
}
