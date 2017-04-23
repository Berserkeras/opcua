// This file was autogenerated from Opc.Ua.NodeSet2.Part13.xml
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

use prelude::*;

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    {
        // Object
        // At the beginning of each interval, retrieve the calculated value from the data points on either side of the requested timestamp.
        let browse_name = "Interpolative";
        let display_name = "Interpolative";
        let node_id = NodeId::new_numeric(0, 2341);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "At the beginning of each interval, retrieve the calculated value from the data points on either side of the requested timestamp."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the average value of the data over the interval.
        let browse_name = "Average";
        let display_name = "Average";
        let node_id = NodeId::new_numeric(0, 2342);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the average value of the data over the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the time weighted average data over the interval using Interpolated Bounding Values.
        let browse_name = "TimeAverage";
        let display_name = "TimeAverage";
        let node_id = NodeId::new_numeric(0, 2343);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the time weighted average data over the interval using Interpolated Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the time weighted average data over the interval using Simple Bounding Values.
        let browse_name = "TimeAverage2";
        let display_name = "TimeAverage2";
        let node_id = NodeId::new_numeric(0, 11285);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the time weighted average data over the interval using Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the total (time integral) of the data over the interval using Interpolated Bounding Values.
        let browse_name = "Total";
        let display_name = "Total";
        let node_id = NodeId::new_numeric(0, 2344);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the total (time integral) of the data over the interval using Interpolated Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the total (time integral) of the data over the interval using Simple Bounding Values.
        let browse_name = "Total2";
        let display_name = "Total2";
        let node_id = NodeId::new_numeric(0, 11304);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the total (time integral) of the data over the interval using Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the minimum raw value in the interval with the timestamp of the start of the interval.
        let browse_name = "Minimum";
        let display_name = "Minimum";
        let node_id = NodeId::new_numeric(0, 2346);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the minimum raw value in the interval with the timestamp of the start of the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the maximum raw value in the interval with the timestamp of the start of the interval.
        let browse_name = "Maximum";
        let display_name = "Maximum";
        let node_id = NodeId::new_numeric(0, 2347);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the maximum raw value in the interval with the timestamp of the start of the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the minimum value in the interval and the Timestamp of the minimum value.
        let browse_name = "MinimumActualTime";
        let display_name = "MinimumActualTime";
        let node_id = NodeId::new_numeric(0, 2348);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the minimum value in the interval and the Timestamp of the minimum value."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the maximum value in the interval and the Timestamp of the maximum value.
        let browse_name = "MaximumActualTime";
        let display_name = "MaximumActualTime";
        let node_id = NodeId::new_numeric(0, 2349);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the maximum value in the interval and the Timestamp of the maximum value."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the difference between the minimum and maximum Value over the interval.
        let browse_name = "Range";
        let display_name = "Range";
        let node_id = NodeId::new_numeric(0, 2350);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the difference between the minimum and maximum Value over the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the minimum value in the interval including the Simple Bounding Values.
        let browse_name = "Minimum2";
        let display_name = "Minimum2";
        let node_id = NodeId::new_numeric(0, 11286);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the minimum value in the interval including the Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the maximum value in the interval including the Simple Bounding Values.
        let browse_name = "Maximum2";
        let display_name = "Maximum2";
        let node_id = NodeId::new_numeric(0, 11287);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the maximum value in the interval including the Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the minimum value with the actual timestamp including the Simple Bounding Values.
        let browse_name = "MinimumActualTime2";
        let display_name = "MinimumActualTime2";
        let node_id = NodeId::new_numeric(0, 11305);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the minimum value with the actual timestamp including the Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the maximum value with the actual timestamp including the Simple Bounding Values.
        let browse_name = "MaximumActualTime2";
        let display_name = "MaximumActualTime2";
        let node_id = NodeId::new_numeric(0, 11306);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the maximum value with the actual timestamp including the Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the difference between the Minimum2 and Maximum2 value over the interval.
        let browse_name = "Range2";
        let display_name = "Range2";
        let node_id = NodeId::new_numeric(0, 11288);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the difference between the Minimum2 and Maximum2 value over the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the number of Annotations in the interval.
        let browse_name = "AnnotationCount";
        let display_name = "AnnotationCount";
        let node_id = NodeId::new_numeric(0, 2351);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the number of Annotations in the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the number of raw values over the interval.
        let browse_name = "Count";
        let display_name = "Count";
        let node_id = NodeId::new_numeric(0, 2352);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the number of raw values over the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the time a Boolean or numeric was in a zero state using Simple Bounding Values.
        let browse_name = "DurationInStateZero";
        let display_name = "DurationInStateZero";
        let node_id = NodeId::new_numeric(0, 11307);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the time a Boolean or numeric was in a zero state using Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the time a Boolean or numeric was in a non-zero state using Simple Bounding Values.
        let browse_name = "DurationInStateNonZero";
        let display_name = "DurationInStateNonZero";
        let node_id = NodeId::new_numeric(0, 11308);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the time a Boolean or numeric was in a non-zero state using Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the number of changes between zero and non-zero that a Boolean or Numeric value experienced in the interval.
        let browse_name = "NumberOfTransitions";
        let display_name = "NumberOfTransitions";
        let node_id = NodeId::new_numeric(0, 2355);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the number of changes between zero and non-zero that a Boolean or Numeric value experienced in the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the value at the beginning of the interval using Interpolated Bounding Values.
        let browse_name = "Start";
        let display_name = "Start";
        let node_id = NodeId::new_numeric(0, 2357);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the value at the beginning of the interval using Interpolated Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the value at the end of the interval using Interpolated Bounding Values.
        let browse_name = "End";
        let display_name = "End";
        let node_id = NodeId::new_numeric(0, 2358);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the value at the end of the interval using Interpolated Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the difference between the Start and End value in the interval.
        let browse_name = "Delta";
        let display_name = "Delta";
        let node_id = NodeId::new_numeric(0, 2359);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the difference between the Start and End value in the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the value at the beginning of the interval using Simple Bounding Values.
        let browse_name = "StartBound";
        let display_name = "StartBound";
        let node_id = NodeId::new_numeric(0, 11505);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the value at the beginning of the interval using Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the value at the end of the interval using Simple Bounding Values.
        let browse_name = "EndBound";
        let display_name = "EndBound";
        let node_id = NodeId::new_numeric(0, 11506);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the value at the end of the interval using Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the difference between the StartBound and EndBound value in the interval.
        let browse_name = "DeltaBounds";
        let display_name = "DeltaBounds";
        let node_id = NodeId::new_numeric(0, 11507);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the difference between the StartBound and EndBound value in the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the total duration of time in the interval during which the data is good.
        let browse_name = "DurationGood";
        let display_name = "DurationGood";
        let node_id = NodeId::new_numeric(0, 2360);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the total duration of time in the interval during which the data is good."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the total duration of time in the interval during which the data is bad.
        let browse_name = "DurationBad";
        let display_name = "DurationBad";
        let node_id = NodeId::new_numeric(0, 2361);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the total duration of time in the interval during which the data is bad."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the percent of data (0 to 100) in the interval which has a good StatusCode.
        let browse_name = "PercentGood";
        let display_name = "PercentGood";
        let node_id = NodeId::new_numeric(0, 2362);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the percent of data (0 to 100) in the interval which has a good StatusCode."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the percent of data (0 to 100) in the interval which has a bad StatusCode.
        let browse_name = "PercentBad";
        let display_name = "PercentBad";
        let node_id = NodeId::new_numeric(0, 2363);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the percent of data (0 to 100) in the interval which has a bad StatusCode."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the worst StatusCode of data in the interval.
        let browse_name = "WorstQuality";
        let display_name = "WorstQuality";
        let node_id = NodeId::new_numeric(0, 2364);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the worst StatusCode of data in the interval."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the worst StatusCode of data in the interval including the Simple Bounding Values.
        let browse_name = "WorstQuality2";
        let display_name = "WorstQuality2";
        let node_id = NodeId::new_numeric(0, 11292);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the worst StatusCode of data in the interval including the Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the standard deviation for the interval for a sample of the population (n-1).
        let browse_name = "StandardDeviationSample";
        let display_name = "StandardDeviationSample";
        let node_id = NodeId::new_numeric(0, 11426);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the standard deviation for the interval for a sample of the population (n-1)."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the standard deviation for the interval for a complete population (n) which includes Simple Bounding Values.
        let browse_name = "StandardDeviationPopulation";
        let display_name = "StandardDeviationPopulation";
        let node_id = NodeId::new_numeric(0, 11427);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the standard deviation for the interval for a complete population (n) which includes Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the variance for the interval as calculated by the StandardDeviationSample.
        let browse_name = "VarianceSample";
        let display_name = "VarianceSample";
        let node_id = NodeId::new_numeric(0, 11428);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the variance for the interval as calculated by the StandardDeviationSample."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // Object
        // Retrieve the variance for the interval as calculated by the StandardDeviationPopulation which includes Simple Bounding Values.
        let browse_name = "VariancePopulation";
        let display_name = "VariancePopulation";
        let node_id = NodeId::new_numeric(0, 11429);
        let node = Object::new_node(&node_id, browse_name, display_name);
        // node.set_description(LocalizedText::new("", "Retrieve the variance for the interval as calculated by the StandardDeviationPopulation which includes Simple Bounding Values."));
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 2340), ReferenceTypeId::HasTypeDefinition);
    }
    {
        // ObjectType
        let browse_name = "AggregateConfigurationType";
        let display_name = "AggregateConfigurationType";
        let node_id = NodeId::new_numeric(0, 11187);
        let node = ObjectType::new_node(&node_id, browse_name, display_name, false);
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 11188), ReferenceTypeId::HasProperty);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 11189), ReferenceTypeId::HasProperty);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 11190), ReferenceTypeId::HasProperty);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 11191), ReferenceTypeId::HasProperty);
        address_space.insert_reference(&NodeId::new_numeric(0, 58), &node_id, ReferenceTypeId::HasSubtype);
    }
    {
        // Variable
        let browse_name = "TreatUncertainAsBad";
        let display_name = "TreatUncertainAsBad";
        let node_id = NodeId::new_numeric(0, 11188);
        let node = Variable::new_node(&node_id, browse_name, display_name, DataTypeId::Boolean, DataValue::null());
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 68), ReferenceTypeId::HasTypeDefinition);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 78), ReferenceTypeId::HasModellingRule);
        address_space.insert_reference(&NodeId::new_numeric(0, 11187), &node_id, ReferenceTypeId::HasProperty);
        address_space.add_organizes(&NodeId::new_numeric(0, 11187), &node_id);
    }
    {
        // Variable
        let browse_name = "PercentDataBad";
        let display_name = "PercentDataBad";
        let node_id = NodeId::new_numeric(0, 11189);
        let node = Variable::new_node(&node_id, browse_name, display_name, DataTypeId::Byte, DataValue::null());
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 68), ReferenceTypeId::HasTypeDefinition);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 78), ReferenceTypeId::HasModellingRule);
        address_space.insert_reference(&NodeId::new_numeric(0, 11187), &node_id, ReferenceTypeId::HasProperty);
        address_space.add_organizes(&NodeId::new_numeric(0, 11187), &node_id);
    }
    {
        // Variable
        let browse_name = "PercentDataGood";
        let display_name = "PercentDataGood";
        let node_id = NodeId::new_numeric(0, 11190);
        let node = Variable::new_node(&node_id, browse_name, display_name, DataTypeId::Byte, DataValue::null());
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 68), ReferenceTypeId::HasTypeDefinition);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 78), ReferenceTypeId::HasModellingRule);
        address_space.insert_reference(&NodeId::new_numeric(0, 11187), &node_id, ReferenceTypeId::HasProperty);
        address_space.add_organizes(&NodeId::new_numeric(0, 11187), &node_id);
    }
    {
        // Variable
        let browse_name = "UseSlopedExtrapolation";
        let display_name = "UseSlopedExtrapolation";
        let node_id = NodeId::new_numeric(0, 11191);
        let node = Variable::new_node(&node_id, browse_name, display_name, DataTypeId::Boolean, DataValue::null());
        address_space.insert(node);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 68), ReferenceTypeId::HasTypeDefinition);
        address_space.insert_reference(&node_id, &NodeId::new_numeric(0, 78), ReferenceTypeId::HasModellingRule);
        address_space.insert_reference(&NodeId::new_numeric(0, 11187), &node_id, ReferenceTypeId::HasProperty);
        address_space.add_organizes(&NodeId::new_numeric(0, 11187), &node_id);
    }
}