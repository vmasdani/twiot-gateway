-- Your SQL goes here
create table sensors (
  id integer primary key autoincrement,
  name text not null,
  device_id integer not null,
  sensor_type_id integer not null,
  serial_number text not null
)
