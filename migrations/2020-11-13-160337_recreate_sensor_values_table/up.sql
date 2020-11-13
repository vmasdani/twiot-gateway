-- Your SQL goes here
drop table sensor_values;
create table sensor_values (
    id integer primary key autoincrement,
    sense_value integer not null,
    max integer not null,
    min integer not null,
    sensor_id integer not null
);