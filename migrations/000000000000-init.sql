CREATE TABLE notes (
    id uuid primary key default uuid_generate_v4(),
    title varchar(255) default null,
    body text not null
);