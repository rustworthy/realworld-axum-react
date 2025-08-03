create table if not exists confirmation_tokens (
  id            integer generated always as identity,
  token         text not null,
  created_at    timestamptz not null default now(),
  expires_at    timestamptz,
  purpose       text, -- e.g.: "EMAIL_CONFIRMATION"
  user_id       uuid
);

create unique index token_purpose_idx on confirmation_tokens (token, purpose, user_id);
