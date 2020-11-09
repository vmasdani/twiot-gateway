-- Your SQL goes here
create table schedules (
  id integer primary key autoincrement,
  hour integer not null,
  minute integer not null
)