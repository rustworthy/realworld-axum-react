CREATE TABLE IF NOT EXISTS "users" (
    user_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    email TEXT COLLATE "case_insensitive" UNIQUE NOT NULL,
    bio TEXT NOT NULL DEFAULT '',
    image TEXT,
    status TEXT,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

SELECT put_creation_mutation_timestamps_guard_on ('users');