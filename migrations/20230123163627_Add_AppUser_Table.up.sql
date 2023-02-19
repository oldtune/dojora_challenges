-- Add up migration script here
create table appuser(
    id uuid primary key not null,
    username text not null,
    password text not null
)