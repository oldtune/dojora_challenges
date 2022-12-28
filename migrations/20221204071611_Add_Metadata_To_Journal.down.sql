-- Add down migration script here
alter table Journals
drop column  created_by text not null,
drop column updated_at bigint not null,
drop column updated_by text not null