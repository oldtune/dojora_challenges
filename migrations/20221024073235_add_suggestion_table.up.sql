-- Add up migration script here
create table Suggestion(
    id uuid primary key not null,
    title text not null,
    description text not null,
    created_at bigint not null
)