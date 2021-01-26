-- Your SQL goes here
create table sensor_types (
  id integer primary key autoincrement,
  name text,
  created_at datetime default current_timestamp,
  updated_at datetime
);

create trigger sensor_types_ts after insert on sensor_types begin
  update sensor_types set updated_at=current_timestamp where id=new.id;
end;