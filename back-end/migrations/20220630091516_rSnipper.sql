-- Add migration script here
create table tags(
    id integer primary key autoincrement,
    tag text not null,
    create_time text not null
);