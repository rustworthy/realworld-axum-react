/*
Inspired by Austin Bonander's setup migration. 
https://github.com/rustworthy/realworld-axum-sqlx/blob/d03a2885b661c8466de24c507099e0e2d66b55bd/migrations/1_setup.sql

We are setting 'updated_at' on each mutation and also making sure they (we, really)
are not trying to modify 'created_at' field.

Usage:
```sql
create table if not exists users(
    user_id     uuid primary key default gen_random_uuid(),
    created_at  timestamptz not null default now(),
    updated_at  timestamptz,
    username    text not null
);
select put_creation_mutation_timestamps_guard_on('users');
```

Updating username in the  table above will trigger `updated_at` field
population, while trying to update `created_at` will cause an exception.
*/

create or replace function guard_creation_mutation_timestamps() returns trigger as
$$
begin
    -- ensure "created_at" is immutable
    if old.created_at != new.created_at then
        raise exception 
        'Cannot change column "created_at" of "%" from "%" to "%". This field is immutable.', 
        tg_table_name, old.created_at, new.created_at;

    end if;
    -- set "update_at" 
    new.updated_at = now();
    return new;
end
$$
language plpgsql;

-- https://www.postgresql.org/docs/8.1/sql-createtrigger.html
create or replace function put_creation_mutation_timestamps_guard_on(tablename regclass) returns void as
$$
begin
    execute format('
        create trigger guard_creation_mutation_timestamps
        before update on %s
        for each row
        when (old.* is distinct from new.*)
        execute function guard_creation_mutation_timestamps();
    ', tablename);
end;
$$
language plpgsql;
