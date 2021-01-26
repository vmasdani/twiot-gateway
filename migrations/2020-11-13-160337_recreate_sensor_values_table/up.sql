-- Your SQL goes here
drop table sensor_values;
create table sensor_values (
    id integer primary key autoincrement,
    sense_value integer,
    max integer,
    min integer,
    sensor_id integer,
    created_at datetime default current_timestamp,
    updated_at datetime
);

create trigger sensor_values_ts after insert on sensor_values begin
  update sensor_values set updated_at=current_timestamp where id=new.id;
end;