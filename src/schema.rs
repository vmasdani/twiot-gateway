table! {
    device_types (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    devices (id) {
        id -> Integer,
        name -> Text,
        serial_number -> Text,
        device_type_id -> Integer,
    }
}

table! {
    schedules (id) {
        id -> Integer,
        hour -> Integer,
        minute -> Integer,
    }
}

table! {
    sensor_types (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    sensors (id) {
        id -> Integer,
        name -> Text,
        device_id -> Integer,
        sensor_type_id -> Integer,
        serial_number -> Text,
    }
}

table! {
    watering_times (id) {
        id -> Integer,
        time -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    device_types,
    devices,
    schedules,
    sensor_types,
    sensors,
    watering_times,
);
