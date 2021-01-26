-- Your SQL goes here
create table devices (
  id integer primary key autoincrement,
  name text,
  serial_number text,
  device_type_id integer,
  created_at datetime default current_timestamp,
  updated_at datetime
);

create trigger devices_ts after insert on devices begin
  update devices set updated_at=current_timestamp where id=new.id;
end;