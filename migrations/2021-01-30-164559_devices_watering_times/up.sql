-- Your SQL goes here
create table device_watering_times (
  id integer primary key autoincrement,
  device_id integer,
  watering_time_id integer,
  created_at datetime default current_timestamp,
  updated_at datetime
);

create trigger device_watering_times_ts after insert on device_watering_times begin
  update device_watering_times set updated_at=current_timestamp where id=new.id;
end;