// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::replicate_key::_replicate_key_output::ReplicateKeyOutputBuilder;

pub use crate::operation::replicate_key::_replicate_key_input::ReplicateKeyInputBuilder;

/// Fluent builder constructing a request to `ReplicateKey`.
///
/// <p>Replicates a multi-Region key into the specified Region. This operation creates a multi-Region replica key based on a multi-Region primary key in a different Region of the same Amazon Web Services partition. You can create multiple replicas of a primary key, but each must be in a different Region. To create a multi-Region primary key, use the <code>CreateKey</code> operation.</p>
/// <p>This operation supports <i>multi-Region keys</i>, an KMS feature that lets you create multiple interoperable KMS keys in different Amazon Web Services Regions. Because these KMS keys have the same key ID, key material, and other metadata, you can use them interchangeably to encrypt data in one Amazon Web Services Region and decrypt it in a different Amazon Web Services Region without re-encrypting the data or making a cross-Region call. For more information about multi-Region keys, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html">Multi-Region keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p>A <i>replica key</i> is a fully-functional KMS key that can be used independently of its primary and peer replica keys. A primary key and its replica keys share properties that make them interoperable. They have the same <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-id-key-id">key ID</a> and key material. They also have the same <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-spec">key spec</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-usage">key usage</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#key-origin">key material origin</a>, and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/rotate-keys.html">automatic key rotation status</a>. KMS automatically synchronizes these shared properties among related multi-Region keys. All other properties of a replica key can differ, including its <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">key policy</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">tags</a>, <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-alias.html">aliases</a>, and <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a>. KMS pricing and quotas for KMS keys apply to each primary key and replica key.</p>
/// <p>When this operation completes, the new replica key has a transient key state of <code>Creating</code>. This key state changes to <code>Enabled</code> (or <code>PendingImport</code>) after a few seconds when the process of creating the new replica key is complete. While the key state is <code>Creating</code>, you can manage key, but you cannot yet use it in cryptographic operations. If you are creating and using the replica key programmatically, retry on <code>KMSInvalidStateException</code> or call <code>DescribeKey</code> to check its <code>KeyState</code> value before using it. For details about the <code>Creating</code> key state, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p>You cannot create more than one replica of a primary key in any Region. If the Region already includes a replica of the key you're trying to replicate, <code>ReplicateKey</code> returns an <code>AlreadyExistsException</code> error. If the key state of the existing replica is <code>PendingDeletion</code>, you can cancel the scheduled key deletion (<code>CancelKeyDeletion</code>) or wait for the key to be deleted. The new replica key you create will have the same <a href="https://docs.aws.amazon.com/kms/latest/developerguide/multi-region-keys-overview.html#mrk-sync-properties">shared properties</a> as the original replica key.</p>
/// <p>The CloudTrail log of a <code>ReplicateKey</code> operation records a <code>ReplicateKey</code> operation in the primary key's Region and a <code>CreateKey</code> operation in the replica key's Region.</p>
/// <p>If you replicate a multi-Region primary key with imported key material, the replica key is created with no key material. You must import the same key material that you imported into the primary key. For details, see <a href="kms/latest/developerguide/multi-region-keys-import.html">Importing key material into multi-Region keys</a> in the <i>Key Management Service Developer Guide</i>.</p>
/// <p>To convert a replica key to a primary key, use the <code>UpdatePrimaryRegion</code> operation.</p> <note>
/// <p> <code>ReplicateKey</code> uses different default values for the <code>KeyPolicy</code> and <code>Tags</code> parameters than those used in the KMS console. For details, see the parameter descriptions.</p>
/// </note>
/// <p> <b>Cross-account use</b>: No. You cannot use this operation to create a replica key in a different Amazon Web Services account. </p>
/// <p> <b>Required permissions</b>: </p>
/// <ul>
/// <li> <p> <code>kms:ReplicateKey</code> on the primary key (in the primary key's Region). Include this permission in the primary key's key policy.</p> </li>
/// <li> <p> <code>kms:CreateKey</code> in an IAM policy in the replica Region.</p> </li>
/// <li> <p>To use the <code>Tags</code> parameter, <code>kms:TagResource</code> in an IAM policy in the replica Region.</p> </li>
/// </ul>
/// <p> <b>Related operations</b> </p>
/// <ul>
/// <li> <p> <code>CreateKey</code> </p> </li>
/// <li> <p> <code>UpdatePrimaryRegion</code> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ReplicateKeyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::replicate_key::builders::ReplicateKeyInputBuilder,
}
impl ReplicateKeyFluentBuilder {
    /// Creates a new `ReplicateKey`.
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
            crate::operation::replicate_key::ReplicateKey,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::replicate_key::ReplicateKeyError>,
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
        crate::operation::replicate_key::ReplicateKeyOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::replicate_key::ReplicateKeyError>,
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
        crate::operation::replicate_key::ReplicateKeyOutput,
        ::aws_smithy_http::result::SdkError<crate::operation::replicate_key::ReplicateKeyError>,
    > {
        self.send_middleware().await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> ::std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::replicate_key::ReplicateKey,
            ::aws_http::retry::AwsResponseRetryClassifier,
        >,
        ::aws_smithy_http::result::SdkError<crate::operation::replicate_key::ReplicateKeyError>,
    > {
        self.customize_middleware().await
    }
    /// <p>Identifies the multi-Region primary key that is being replicated. To determine whether a KMS key is a multi-Region primary key, use the <code>DescribeKey</code> operation to check the value of the <code>MultiRegionKeyType</code> property.</p>
    /// <p>Specify the key ID or key ARN of a multi-Region primary key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>Identifies the multi-Region primary key that is being replicated. To determine whether a KMS key is a multi-Region primary key, use the <code>DescribeKey</code> operation to check the value of the <code>MultiRegionKeyType</code> property.</p>
    /// <p>Specify the key ID or key ARN of a multi-Region primary key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li> <p>Key ID: <code>mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li>
    /// <li> <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/mrk-1234abcd12ab34cd56ef1234567890ab</code> </p> </li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>The Region ID of the Amazon Web Services Region for this replica key. </p>
    /// <p>Enter the Region ID, such as <code>us-east-1</code> or <code>ap-southeast-2</code>. For a list of Amazon Web Services Regions in which KMS is supported, see <a href="https://docs.aws.amazon.com/general/latest/gr/kms.html#kms_region">KMS service endpoints</a> in the <i>Amazon Web Services General Reference</i>.</p> <note>
    /// <p>HMAC KMS keys are not supported in all Amazon Web Services Regions. If you try to replicate an HMAC KMS key in an Amazon Web Services Region in which HMAC keys are not supported, the <code>ReplicateKey</code> operation returns an <code>UnsupportedOperationException</code>. For a list of Regions in which HMAC KMS keys are supported, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// </note>
    /// <p>The replica must be in a different Amazon Web Services Region than its primary key and other replicas of that primary key, but in the same Amazon Web Services partition. KMS must be available in the replica Region. If the Region is not enabled by default, the Amazon Web Services account must be enabled in the Region. For information about Amazon Web Services partitions, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. For information about enabling and disabling Regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">Enabling a Region</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-disable">Disabling a Region</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn replica_region(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.replica_region(input.into());
        self
    }
    /// <p>The Region ID of the Amazon Web Services Region for this replica key. </p>
    /// <p>Enter the Region ID, such as <code>us-east-1</code> or <code>ap-southeast-2</code>. For a list of Amazon Web Services Regions in which KMS is supported, see <a href="https://docs.aws.amazon.com/general/latest/gr/kms.html#kms_region">KMS service endpoints</a> in the <i>Amazon Web Services General Reference</i>.</p> <note>
    /// <p>HMAC KMS keys are not supported in all Amazon Web Services Regions. If you try to replicate an HMAC KMS key in an Amazon Web Services Region in which HMAC keys are not supported, the <code>ReplicateKey</code> operation returns an <code>UnsupportedOperationException</code>. For a list of Regions in which HMAC KMS keys are supported, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/hmac.html">HMAC keys in KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// </note>
    /// <p>The replica must be in a different Amazon Web Services Region than its primary key and other replicas of that primary key, but in the same Amazon Web Services partition. KMS must be available in the replica Region. If the Region is not enabled by default, the Amazon Web Services account must be enabled in the Region. For information about Amazon Web Services partitions, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs)</a> in the <i>Amazon Web Services General Reference</i>. For information about enabling and disabling Regions, see <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-enable">Enabling a Region</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/rande-manage.html#rande-manage-disable">Disabling a Region</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_replica_region(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_replica_region(input);
        self
    }
    /// <p>The key policy to attach to the KMS key. This parameter is optional. If you do not provide a key policy, KMS attaches the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default">default key policy</a> to the KMS key.</p>
    /// <p>The key policy is not a shared property of multi-Region keys. You can specify the same key policy or a different key policy for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>
    /// <p>If you provide a key policy, it must meet the following criteria:</p>
    /// <ul>
    /// <li> <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p> </li>
    /// <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p> </li>
    /// </ul>
    /// <p>A key policy document can include only the following characters:</p>
    /// <ul>
    /// <li> <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p> </li>
    /// <li> <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p> </li>
    /// <li> <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p> </li>
    /// </ul>
    /// <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>. For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy(input.into());
        self
    }
    /// <p>The key policy to attach to the KMS key. This parameter is optional. If you do not provide a key policy, KMS attaches the <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html#key-policy-default">default key policy</a> to the KMS key.</p>
    /// <p>The key policy is not a shared property of multi-Region keys. You can specify the same key policy or a different key policy for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>
    /// <p>If you provide a key policy, it must meet the following criteria:</p>
    /// <ul>
    /// <li> <p>The key policy must allow the calling principal to make a subsequent <code>PutKeyPolicy</code> request on the KMS key. This reduces the risk that the KMS key becomes unmanageable. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>. (To omit this condition, set <code>BypassPolicyLockoutSafetyCheck</code> to true.)</p> </li>
    /// <li> <p>Each statement in the key policy must contain one or more principals. The principals in the key policy must exist and be visible to KMS. When you create a new Amazon Web Services principal, you might need to enforce a delay before including the new principal in a key policy because the new principal might not be immediately visible to KMS. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/troubleshoot_general.html#troubleshoot_general_eventual-consistency">Changes that I make are not always immediately visible</a> in the <i>Amazon Web Services Identity and Access Management User Guide</i>.</p> </li>
    /// </ul>
    /// <p>A key policy document can include only the following characters:</p>
    /// <ul>
    /// <li> <p>Printable ASCII characters from the space character (<code>\u0020</code>) through the end of the ASCII character range.</p> </li>
    /// <li> <p>Printable characters in the Basic Latin and Latin-1 Supplement character set (through <code>\u00FF</code>).</p> </li>
    /// <li> <p>The tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) special characters</p> </li>
    /// </ul>
    /// <p>For information about key policies, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policies.html">Key policies in KMS</a> in the <i>Key Management Service Developer Guide</i>. For help writing and formatting a JSON policy document, see the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies.html">IAM JSON Policy Reference</a> in the <i> <i>Identity and Access Management User Guide</i> </i>.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy(input);
        self
    }
    /// <p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p> <important>
    /// <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// </important>
    /// <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the KMS key.</p>
    pub fn bypass_policy_lockout_safety_check(mut self, input: bool) -> Self {
        self.inner = self.inner.bypass_policy_lockout_safety_check(input);
        self
    }
    /// <p>Skips ("bypasses") the key policy lockout safety check. The default value is false.</p> <important>
    /// <p>Setting this value to true increases the risk that the KMS key becomes unmanageable. Do not set this value to true indiscriminately.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-policy-default.html#prevent-unmanageable-key">Default key policy</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// </important>
    /// <p>Use this parameter only when you intend to prevent the principal that is making the request from making a subsequent <code>PutKeyPolicy</code> request on the KMS key.</p>
    pub fn set_bypass_policy_lockout_safety_check(
        mut self,
        input: ::std::option::Option<bool>,
    ) -> Self {
        self.inner = self.inner.set_bypass_policy_lockout_safety_check(input);
        self
    }
    /// <p>A description of the KMS key. The default value is an empty string (no description).</p> <important>
    /// <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
    /// </important>
    /// <p>The description is not a shared property of multi-Region keys. You can specify the same description or a different description for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the KMS key. The default value is an empty string (no description).</p> <important>
    /// <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
    /// </important>
    /// <p>The description is not a shared property of multi-Region keys. You can specify the same description or a different description for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Assigns one or more tags to the replica key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the <code>TagResource</code> operation.</p> <important>
    /// <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
    /// </important> <note>
    /// <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// </note>
    /// <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p>
    /// <p>Tags are not a shared property of multi-Region keys. You can specify the same tags or different tags for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>
    /// <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p>
    /// <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging Keys</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Assigns one or more tags to the replica key. Use this parameter to tag the KMS key when it is created. To tag an existing KMS key, use the <code>TagResource</code> operation.</p> <important>
    /// <p>Do not include confidential or sensitive information in this field. This field may be displayed in plaintext in CloudTrail logs and other output.</p>
    /// </important> <note>
    /// <p>Tagging or untagging a KMS key can allow or deny permission to the KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/abac.html">ABAC for KMS</a> in the <i>Key Management Service Developer Guide</i>.</p>
    /// </note>
    /// <p>To use this parameter, you must have <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:TagResource</a> permission in an IAM policy.</p>
    /// <p>Tags are not a shared property of multi-Region keys. You can specify the same tags or different tags for each key in a set of related multi-Region keys. KMS does not synchronize this property.</p>
    /// <p>Each tag consists of a tag key and a tag value. Both the tag key and the tag value are required, but the tag value can be an empty (null) string. You cannot have more than one tag on a KMS key with the same tag key. If you specify an existing tag key with a different tag value, KMS replaces the current tag value with the specified one.</p>
    /// <p>When you add tags to an Amazon Web Services resource, Amazon Web Services generates a cost allocation report with usage and costs aggregated by tags. Tags can also be used to control access to a KMS key. For details, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/tagging-keys.html">Tagging Keys</a>.</p>
    pub fn set_tags(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
}
