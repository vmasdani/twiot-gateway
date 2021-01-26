-- Your SQL goes here
create table sensors (
  id integer primary key autoincrement,
  name text,
  device_id integer,
  sensor_type_id integer,
  serial_number text,
  created_at datetime default current_timestamp,
  updated_at datetime
);


create trigger sensors_ts after insert on sensors begin
  update sensors set updated_at=current_timestamp where id=new.id;
end;