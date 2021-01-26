-- Your SQL goes here
create table watering_times (
  id integer primary key autoincrement,
  time integer,
  created_at datetime default current_timestamp,
  updated_at datetime
);

create trigger watering_times_ts after insert on watering_times begin
  update watering_times set updated_at=current_timestamp where id=new.id;
end;