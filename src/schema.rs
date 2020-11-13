table! {
    device_types (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    devices (id) {
        id -> Nullable<Integer>,
        name -> Text,
        serial_number -> Text,
        device_type_id -> Integer,
    }
}

table! {
    schedules (id) {
        id -> Nullable<Integer>,
        hour -> Integer,
        minute -> Integer,
    }
}

table! {
    sensor_types (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    sensor_values (id) {
        id -> Nullable<Integer>,
        sense_value -> Integer,
        max -> Integer,
        min -> Integer,
        sensor_id -> Integer,
    }
}

table! {
    sensors (id) {
        id -> Nullable<Integer>,
        name -> Text,
        device_id -> Integer,
        sensor_type_id -> Integer,
        serial_number -> Text,
    }
}

table! {
    watering_times (id) {
        id -> Nullable<Integer>,
        time -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    device_types,
    devices,
    schedules,
    sensor_types,
    sensor_values,
    sensors,
    watering_times,
);
