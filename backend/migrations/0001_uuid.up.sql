/*
Exposes `uuid_generate_v4` procedure to generate identifiers.

An alternative would be to use `gen_random_uuid` from `pgcrypto` module,
in which case we would do a similar migration but for `pgcrypto` exntension.
    
See module docs here:
 - https://www.postgresql.org/docs/current/uuid-ossp.html
 - https://www.timescale.com/learn/postgresql-extensions-uuid-ossp
*/
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";