table! {
    device_schedules (id) {
        id -> Nullable<Integer>,
        device_id -> Nullable<Integer>,
        schedule_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    device_types (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    device_watering_times (id) {
        id -> Nullable<Integer>,
        device_id -> Nullable<Integer>,
        watering_time_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    devices (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        serial_number -> Nullable<Text>,
        device_type_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        mac -> Nullable<Text>,
        ip -> Nullable<Text>,
    }
}

table! {
    schedules (id) {
        id -> Nullable<Integer>,
        hour -> Nullable<Integer>,
        minute -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    sensor_types (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    sensor_values (id) {
        id -> Nullable<Integer>,
        sense_value -> Nullable<Integer>,
        max -> Nullable<Integer>,
        min -> Nullable<Integer>,
        sensor_id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    sensors (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
        device_id -> Nullable<Integer>,
        sensor_type_id -> Nullable<Integer>,
        serial_number -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    watering_times (id) {
        id -> Nullable<Integer>,
        time -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    device_schedules,
    device_types,
    device_watering_times,
    devices,
    schedules,
    sensor_types,
    sensor_values,
    sensors,
    watering_times,
);
