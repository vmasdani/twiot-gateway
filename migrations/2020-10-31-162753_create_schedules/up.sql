-- Your SQL goes here
create table schedules (
  id integer primary key autoincrement,
  hour integer,
  minute integer,
  created_at datetime default current_timestamp,
  updated_at datetime
);

create trigger schedules_ts after insert on schedules begin
  update schedules set updated_at=current_timestamp where id=new.id;
end;