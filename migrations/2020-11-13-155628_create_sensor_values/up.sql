-- Your SQL goes here
create table sensor_values (
    id integer primary key autoincrement,
    sense_value integer not null,
    max integer not null,
    min integer not null
)