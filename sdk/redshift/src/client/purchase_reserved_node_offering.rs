// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PurchaseReservedNodeOffering`](crate::operation::purchase_reserved_node_offering::builders::PurchaseReservedNodeOfferingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`reserved_node_offering_id(impl ::std::convert::Into<String>)`](crate::operation::purchase_reserved_node_offering::builders::PurchaseReservedNodeOfferingFluentBuilder::reserved_node_offering_id) / [`set_reserved_node_offering_id(Option<String>)`](crate::operation::purchase_reserved_node_offering::builders::PurchaseReservedNodeOfferingFluentBuilder::set_reserved_node_offering_id): <p>The unique identifier of the reserved node offering you want to purchase.</p>
    ///   - [`node_count(i32)`](crate::operation::purchase_reserved_node_offering::builders::PurchaseReservedNodeOfferingFluentBuilder::node_count) / [`set_node_count(Option<i32>)`](crate::operation::purchase_reserved_node_offering::builders::PurchaseReservedNodeOfferingFluentBuilder::set_node_count): <p>The number of reserved nodes that you want to purchase.</p>  <p>Default: <code>1</code> </p>
    /// - On success, responds with [`PurchaseReservedNodeOfferingOutput`](crate::operation::purchase_reserved_node_offering::PurchaseReservedNodeOfferingOutput) with field(s):
    ///   - [`reserved_node(Option<ReservedNode>)`](crate::operation::purchase_reserved_node_offering::PurchaseReservedNodeOfferingOutput::reserved_node): <p>Describes a reserved node. You can call the <code>DescribeReservedNodeOfferings</code> API to obtain the available reserved node offerings. </p>
    /// - On failure, responds with [`SdkError<PurchaseReservedNodeOfferingError>`](crate::operation::purchase_reserved_node_offering::PurchaseReservedNodeOfferingError)
    pub fn purchase_reserved_node_offering(&self) -> crate::operation::purchase_reserved_node_offering::builders::PurchaseReservedNodeOfferingFluentBuilder{
        crate::operation::purchase_reserved_node_offering::builders::PurchaseReservedNodeOfferingFluentBuilder::new(self.handle.clone())
    }
}
