CREATE TABLE IF NOT EXISTS "follows" (
    following_user_id     UUID NOT NULL REFERENCES "users" (user_id) ON DELETE CASCADE,
    followed_user_id      UUID NOT NULL REFERENCES "users" (user_id) ON DELETE CASCADE,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ,

    CONSTRAINT "follows_no_self_follow" CHECK (followed_user_id != following_user_id),

    PRIMARY KEY (following_user_id, followed_user_id)
);

SELECT put_creation_mutation_timestamps_guard_on('follows');

