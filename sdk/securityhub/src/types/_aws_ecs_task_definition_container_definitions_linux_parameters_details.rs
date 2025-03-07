// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>&gt;Linux-specific modifications that are applied to the container, such as Linux kernel capabilities.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails {
    /// <p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker.</p>
    #[doc(hidden)]
    pub capabilities: ::std::option::Option<
        crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails,
    >,
    /// <p>The host devices to expose to the container.</p>
    #[doc(hidden)]
    pub devices: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails,
        >,
    >,
    /// <p>Whether to run an <code>init</code> process inside the container that forwards signals and reaps processes. </p>
    #[doc(hidden)]
    pub init_process_enabled: bool,
    /// <p>The total amount of swap memory (in MiB) that a container can use.</p>
    #[doc(hidden)]
    pub max_swap: i32,
    /// <p>The value for the size (in MiB) of the <b>/dev/shm</b> volume.</p>
    #[doc(hidden)]
    pub shared_memory_size: i32,
    /// <p>Configures the container's memory swappiness behavior. Determines how aggressively pages are swapped. The higher the value, the more aggressive the swappiness. The default is 60.</p>
    #[doc(hidden)]
    pub swappiness: i32,
    /// <p>The container path, mount options, and size (in MiB) of the tmpfs mount.</p>
    #[doc(hidden)]
    pub tmpfs: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails,
        >,
    >,
}
impl AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails {
    /// <p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker.</p>
    pub fn capabilities(
        &self,
    ) -> ::std::option::Option<
        &crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails,
    > {
        self.capabilities.as_ref()
    }
    /// <p>The host devices to expose to the container.</p>
    pub fn devices(
        &self,
    ) -> ::std::option::Option<
        &[crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails],
    > {
        self.devices.as_deref()
    }
    /// <p>Whether to run an <code>init</code> process inside the container that forwards signals and reaps processes. </p>
    pub fn init_process_enabled(&self) -> bool {
        self.init_process_enabled
    }
    /// <p>The total amount of swap memory (in MiB) that a container can use.</p>
    pub fn max_swap(&self) -> i32 {
        self.max_swap
    }
    /// <p>The value for the size (in MiB) of the <b>/dev/shm</b> volume.</p>
    pub fn shared_memory_size(&self) -> i32 {
        self.shared_memory_size
    }
    /// <p>Configures the container's memory swappiness behavior. Determines how aggressively pages are swapped. The higher the value, the more aggressive the swappiness. The default is 60.</p>
    pub fn swappiness(&self) -> i32 {
        self.swappiness
    }
    /// <p>The container path, mount options, and size (in MiB) of the tmpfs mount.</p>
    pub fn tmpfs(
        &self,
    ) -> ::std::option::Option<
        &[crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails],
    > {
        self.tmpfs.as_deref()
    }
}
impl AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails {
    /// Creates a new builder-style object to manufacture [`AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails`](crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails).
    pub fn builder(
    ) -> crate::types::builders::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetailsBuilder
    {
        crate::types::builders::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetailsBuilder::default()
    }
}

/// A builder for [`AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails`](crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetailsBuilder {
    pub(crate) capabilities: ::std::option::Option<
        crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails,
    >,
    pub(crate) devices: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails,
        >,
    >,
    pub(crate) init_process_enabled: ::std::option::Option<bool>,
    pub(crate) max_swap: ::std::option::Option<i32>,
    pub(crate) shared_memory_size: ::std::option::Option<i32>,
    pub(crate) swappiness: ::std::option::Option<i32>,
    pub(crate) tmpfs: ::std::option::Option<
        ::std::vec::Vec<
            crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails,
        >,
    >,
}
impl AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetailsBuilder {
    /// <p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker.</p>
    pub fn capabilities(
        mut self,
        input: crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails,
    ) -> Self {
        self.capabilities = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker.</p>
    pub fn set_capabilities(
        mut self,
        input: ::std::option::Option<crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersCapabilitiesDetails>,
    ) -> Self {
        self.capabilities = input;
        self
    }
    /// Appends an item to `devices`.
    ///
    /// To override the contents of this collection use [`set_devices`](Self::set_devices).
    ///
    /// <p>The host devices to expose to the container.</p>
    pub fn devices(
        mut self,
        input: crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails,
    ) -> Self {
        let mut v = self.devices.unwrap_or_default();
        v.push(input);
        self.devices = ::std::option::Option::Some(v);
        self
    }
    /// <p>The host devices to expose to the container.</p>
    pub fn set_devices(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDevicesDetails,
            >,
        >,
    ) -> Self {
        self.devices = input;
        self
    }
    /// <p>Whether to run an <code>init</code> process inside the container that forwards signals and reaps processes. </p>
    pub fn init_process_enabled(mut self, input: bool) -> Self {
        self.init_process_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Whether to run an <code>init</code> process inside the container that forwards signals and reaps processes. </p>
    pub fn set_init_process_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.init_process_enabled = input;
        self
    }
    /// <p>The total amount of swap memory (in MiB) that a container can use.</p>
    pub fn max_swap(mut self, input: i32) -> Self {
        self.max_swap = ::std::option::Option::Some(input);
        self
    }
    /// <p>The total amount of swap memory (in MiB) that a container can use.</p>
    pub fn set_max_swap(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_swap = input;
        self
    }
    /// <p>The value for the size (in MiB) of the <b>/dev/shm</b> volume.</p>
    pub fn shared_memory_size(mut self, input: i32) -> Self {
        self.shared_memory_size = ::std::option::Option::Some(input);
        self
    }
    /// <p>The value for the size (in MiB) of the <b>/dev/shm</b> volume.</p>
    pub fn set_shared_memory_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.shared_memory_size = input;
        self
    }
    /// <p>Configures the container's memory swappiness behavior. Determines how aggressively pages are swapped. The higher the value, the more aggressive the swappiness. The default is 60.</p>
    pub fn swappiness(mut self, input: i32) -> Self {
        self.swappiness = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configures the container's memory swappiness behavior. Determines how aggressively pages are swapped. The higher the value, the more aggressive the swappiness. The default is 60.</p>
    pub fn set_swappiness(mut self, input: ::std::option::Option<i32>) -> Self {
        self.swappiness = input;
        self
    }
    /// Appends an item to `tmpfs`.
    ///
    /// To override the contents of this collection use [`set_tmpfs`](Self::set_tmpfs).
    ///
    /// <p>The container path, mount options, and size (in MiB) of the tmpfs mount.</p>
    pub fn tmpfs(
        mut self,
        input: crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails,
    ) -> Self {
        let mut v = self.tmpfs.unwrap_or_default();
        v.push(input);
        self.tmpfs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The container path, mount options, and size (in MiB) of the tmpfs mount.</p>
    pub fn set_tmpfs(
        mut self,
        input: ::std::option::Option<
            ::std::vec::Vec<
                crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersTmpfsDetails,
            >,
        >,
    ) -> Self {
        self.tmpfs = input;
        self
    }
    /// Consumes the builder and constructs a [`AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails`](crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails).
    pub fn build(
        self,
    ) -> crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails {
        crate::types::AwsEcsTaskDefinitionContainerDefinitionsLinuxParametersDetails {
            capabilities: self.capabilities,
            devices: self.devices,
            init_process_enabled: self.init_process_enabled.unwrap_or_default(),
            max_swap: self.max_swap.unwrap_or_default(),
            shared_memory_size: self.shared_memory_size.unwrap_or_default(),
            swappiness: self.swappiness.unwrap_or_default(),
            tmpfs: self.tmpfs,
        }
    }
}
