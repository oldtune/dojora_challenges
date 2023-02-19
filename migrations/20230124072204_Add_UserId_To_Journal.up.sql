-- Add up migration script here
alter table journals
add userid uuid not null;
