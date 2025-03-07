// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the data tiering policy for an ONTAP volume. When enabled, Amazon FSx for ONTAP's intelligent tiering automatically transitions a volume's data between the file system's primary storage and capacity pool storage based on your access patterns.</p>
/// <p>Valid tiering policies are the following:</p>
/// <ul>
/// <li> <p> <code>SNAPSHOT_ONLY</code> - (Default value) moves cold snapshots to the capacity pool storage tier.</p> </li>
/// </ul>
/// <ul>
/// <li> <p> <code>AUTO</code> - moves cold user data and snapshots to the capacity pool storage tier based on your access patterns.</p> </li>
/// </ul>
/// <ul>
/// <li> <p> <code>ALL</code> - moves all user data blocks in both the active file system and Snapshot copies to the storage pool tier.</p> </li>
/// </ul>
/// <ul>
/// <li> <p> <code>NONE</code> - keeps a volume's data in the primary storage tier, preventing it from being moved to the capacity pool tier.</p> </li>
/// </ul>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TieringPolicy {
    /// <p>Specifies the number of days that user data in a volume must remain inactive before it is considered "cold" and moved to the capacity pool. Used with the <code>AUTO</code> and <code>SNAPSHOT_ONLY</code> tiering policies. Enter a whole number between 2 and 183. Default values are 31 days for <code>AUTO</code> and 2 days for <code>SNAPSHOT_ONLY</code>.</p>
    #[doc(hidden)]
    pub cooling_period: ::std::option::Option<i32>,
    /// <p>Specifies the tiering policy used to transition data. Default value is <code>SNAPSHOT_ONLY</code>.</p>
    /// <ul>
    /// <li> <p> <code>SNAPSHOT_ONLY</code> - moves cold snapshots to the capacity pool storage tier.</p> </li>
    /// <li> <p> <code>AUTO</code> - moves cold user data and snapshots to the capacity pool storage tier based on your access patterns.</p> </li>
    /// <li> <p> <code>ALL</code> - moves all user data blocks in both the active file system and Snapshot copies to the storage pool tier.</p> </li>
    /// <li> <p> <code>NONE</code> - keeps a volume's data in the primary storage tier, preventing it from being moved to the capacity pool tier.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub name: ::std::option::Option<crate::types::TieringPolicyName>,
}
impl TieringPolicy {
    /// <p>Specifies the number of days that user data in a volume must remain inactive before it is considered "cold" and moved to the capacity pool. Used with the <code>AUTO</code> and <code>SNAPSHOT_ONLY</code> tiering policies. Enter a whole number between 2 and 183. Default values are 31 days for <code>AUTO</code> and 2 days for <code>SNAPSHOT_ONLY</code>.</p>
    pub fn cooling_period(&self) -> ::std::option::Option<i32> {
        self.cooling_period
    }
    /// <p>Specifies the tiering policy used to transition data. Default value is <code>SNAPSHOT_ONLY</code>.</p>
    /// <ul>
    /// <li> <p> <code>SNAPSHOT_ONLY</code> - moves cold snapshots to the capacity pool storage tier.</p> </li>
    /// <li> <p> <code>AUTO</code> - moves cold user data and snapshots to the capacity pool storage tier based on your access patterns.</p> </li>
    /// <li> <p> <code>ALL</code> - moves all user data blocks in both the active file system and Snapshot copies to the storage pool tier.</p> </li>
    /// <li> <p> <code>NONE</code> - keeps a volume's data in the primary storage tier, preventing it from being moved to the capacity pool tier.</p> </li>
    /// </ul>
    pub fn name(&self) -> ::std::option::Option<&crate::types::TieringPolicyName> {
        self.name.as_ref()
    }
}
impl TieringPolicy {
    /// Creates a new builder-style object to manufacture [`TieringPolicy`](crate::types::TieringPolicy).
    pub fn builder() -> crate::types::builders::TieringPolicyBuilder {
        crate::types::builders::TieringPolicyBuilder::default()
    }
}

/// A builder for [`TieringPolicy`](crate::types::TieringPolicy).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TieringPolicyBuilder {
    pub(crate) cooling_period: ::std::option::Option<i32>,
    pub(crate) name: ::std::option::Option<crate::types::TieringPolicyName>,
}
impl TieringPolicyBuilder {
    /// <p>Specifies the number of days that user data in a volume must remain inactive before it is considered "cold" and moved to the capacity pool. Used with the <code>AUTO</code> and <code>SNAPSHOT_ONLY</code> tiering policies. Enter a whole number between 2 and 183. Default values are 31 days for <code>AUTO</code> and 2 days for <code>SNAPSHOT_ONLY</code>.</p>
    pub fn cooling_period(mut self, input: i32) -> Self {
        self.cooling_period = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the number of days that user data in a volume must remain inactive before it is considered "cold" and moved to the capacity pool. Used with the <code>AUTO</code> and <code>SNAPSHOT_ONLY</code> tiering policies. Enter a whole number between 2 and 183. Default values are 31 days for <code>AUTO</code> and 2 days for <code>SNAPSHOT_ONLY</code>.</p>
    pub fn set_cooling_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.cooling_period = input;
        self
    }
    /// <p>Specifies the tiering policy used to transition data. Default value is <code>SNAPSHOT_ONLY</code>.</p>
    /// <ul>
    /// <li> <p> <code>SNAPSHOT_ONLY</code> - moves cold snapshots to the capacity pool storage tier.</p> </li>
    /// <li> <p> <code>AUTO</code> - moves cold user data and snapshots to the capacity pool storage tier based on your access patterns.</p> </li>
    /// <li> <p> <code>ALL</code> - moves all user data blocks in both the active file system and Snapshot copies to the storage pool tier.</p> </li>
    /// <li> <p> <code>NONE</code> - keeps a volume's data in the primary storage tier, preventing it from being moved to the capacity pool tier.</p> </li>
    /// </ul>
    pub fn name(mut self, input: crate::types::TieringPolicyName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the tiering policy used to transition data. Default value is <code>SNAPSHOT_ONLY</code>.</p>
    /// <ul>
    /// <li> <p> <code>SNAPSHOT_ONLY</code> - moves cold snapshots to the capacity pool storage tier.</p> </li>
    /// <li> <p> <code>AUTO</code> - moves cold user data and snapshots to the capacity pool storage tier based on your access patterns.</p> </li>
    /// <li> <p> <code>ALL</code> - moves all user data blocks in both the active file system and Snapshot copies to the storage pool tier.</p> </li>
    /// <li> <p> <code>NONE</code> - keeps a volume's data in the primary storage tier, preventing it from being moved to the capacity pool tier.</p> </li>
    /// </ul>
    pub fn set_name(
        mut self,
        input: ::std::option::Option<crate::types::TieringPolicyName>,
    ) -> Self {
        self.name = input;
        self
    }
    /// Consumes the builder and constructs a [`TieringPolicy`](crate::types::TieringPolicy).
    pub fn build(self) -> crate::types::TieringPolicy {
        crate::types::TieringPolicy {
            cooling_period: self.cooling_period,
            name: self.name,
        }
    }
}
