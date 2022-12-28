-- Add up migration script here
alter table Journals
add column created_by text not null,
add column updated_at bigint not null,
add column updated_by text not null