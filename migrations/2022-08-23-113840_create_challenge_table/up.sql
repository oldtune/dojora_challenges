-- Your SQL goes here
create table Challenge(
    Id uuid not null primary key,
    Title text not null,
    Description text null
)