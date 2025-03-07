// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_calculate_route_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::calculate_route::CalculateRouteInput,
) -> Result<(), ::aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.car_mode_options {
        #[allow(unused_mut)]
        let mut object_2 = object.key("CarModeOptions").start_object();
        crate::protocol_serde::shape_calculate_route_car_mode_options::ser_calculate_route_car_mode_options(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.depart_now {
        object.key("DepartNow").boolean(*var_3);
    }
    if let Some(var_4) = &input.departure_position {
        let mut array_5 = object.key("DeparturePosition").start_array();
        for item_6 in var_4 {
            {
                array_5.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::Float((*item_6).into()),
                );
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.departure_time {
        object
            .key("DepartureTime")
            .date_time(var_7, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    if let Some(var_8) = &input.destination_position {
        let mut array_9 = object.key("DestinationPosition").start_array();
        for item_10 in var_8 {
            {
                array_9.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::Float((*item_10).into()),
                );
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.distance_unit {
        object.key("DistanceUnit").string(var_11.as_str());
    }
    if let Some(var_12) = &input.include_leg_geometry {
        object.key("IncludeLegGeometry").boolean(*var_12);
    }
    if let Some(var_13) = &input.travel_mode {
        object.key("TravelMode").string(var_13.as_str());
    }
    if let Some(var_14) = &input.truck_mode_options {
        #[allow(unused_mut)]
        let mut object_15 = object.key("TruckModeOptions").start_object();
        crate::protocol_serde::shape_calculate_route_truck_mode_options::ser_calculate_route_truck_mode_options(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.waypoint_positions {
        let mut array_17 = object.key("WaypointPositions").start_array();
        for item_18 in var_16 {
            {
                let mut array_19 = array_17.value().start_array();
                for item_20 in item_18 {
                    {
                        array_19.value().number(
                            #[allow(clippy::useless_conversion)]
                            ::aws_smithy_types::Number::Float((*item_20).into()),
                        );
                    }
                }
                array_19.finish();
            }
        }
        array_17.finish();
    }
    Ok(())
}
