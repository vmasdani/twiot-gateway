-- Your SQL goes here
create table devices (
  id integer primary key autoincrement,
  name text not null,
  serial_number text not null,
  device_type_id integer not null
)