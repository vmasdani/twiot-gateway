-- Your SQL goes here-- Your SQL goes here
create table device_schedules (
  id integer primary key autoincrement,
  device_id integer,
  schedule_id integer,
  created_at datetime default current_timestamp,
  updated_at datetime
);

create trigger device_schedules_ts after insert on device_schedules begin
  update device_schedules set updated_at=current_timestamp where id=new.id;
end;