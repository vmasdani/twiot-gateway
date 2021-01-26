-- Your SQL goes here
create table device_types (
  id integer primary key autoincrement,
  name text,
  created_at datetime default current_timestamp,
  updated_at datetime
);

create trigger device_types_ts after insert on device_types begin
  update device_types set updated_at=current_timestamp where id=new.id;
end;