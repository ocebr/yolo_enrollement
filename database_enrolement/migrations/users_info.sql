-- Add migration script here
create table IF NOT EXISTS users_info(
    id uuid not null primary key,
    full_name varchar null,
    bio varchar null,
    image varchar null,
    -- email_verified
    -- active
    created_at timestamp not null  default current_timestamp,
    updated_at timestamp not null  default current_timestamp
);