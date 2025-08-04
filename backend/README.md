# Axum & Sqlx powered backend of `realworld-axum-react` app

## Spec

Find `RealWorld` API specification [on the project's website][1].

The `RealWorld` repository also includes a `Postman` collection that
_should_ be used to run tests on CI with [`Newman`][2].

## Setup

Make sure you got [`cargo`][4], [`GNU Make`][5], and [`docker`][6] installed.
Change your working directory to `backend` and hit:

```console
make setup
```

You should now be able to start the back-end in watch mode with:

```console
make watch
```

Run end-to-end tests with:

```console
make test/e2e
```

NB! Make sure you got `docker` installed: for each of tests we are launching a
dedicated [test container][3] with PostgreSQL.

## E2E Browser Tests

Prerequisites:

- [google-chrome][7]
- [chromedriver][8]
- frontend project [setup](../frontend/README.md)

The in the `backend` directory, hit:

```console
make test/e2e/browser
HEADLESS=1 make test/e2e/browser # to run in headless mode
```

## Development

### Code changes

Make sure the database application is running (`make db/start`) and use `make watch`
to automatically restart the backend application once there is a file change.

### Database migrations

To add new migrations files, hit:

```
make db/migrate/add name=<you_migration_name>
```

For example, adding confirmation tokens will resemble:

```console
$ make db/migrate/add name="confirmation_tokens"
sqlx migrate add -rs confirmation_tokens
Creating migrations/0004_confirmation_tokens.up.sql
Creating migrations/0004_confirmation_tokens.down.sql
```

In development mode, the default is to apply migrations whenever the application
starts, but you can opt out of it by providing `MIGRATE=false` environment variable.
In this case you will need to apply migrations yourself which can be achieved with:

```console
make db/migrate/run
```

The above command will apply all pending migrations.
To revert the latest migration, hit:

```console
make db/migrate/revert
```

Note that those `make` commands are thin wrappers over the `sqlx` CLI commands,
that just make sure to provide the correct connection string (see [`Makefile](./Makefile)).
I.e. you can achieve all of that and even more - if needed - using the`sqlx` tool
directly. Most of the time, though, we want to create a new migration file,
apply it, then revert it as a sanity check, and then re-apply it.

<!-- -------------------------------- LINKS -------------------------------- -->
[1]: https://github.com/gothinkster/realworld/blob/09e8fa29ef0ee39fa5d1caecfa0b4e5f090bbe92/api/openapi.yml
[2]: https://learning.postman.com/docs/collections/using-newman-cli/command-line-integration-with-newman/
[3]: https://testcontainers.com/?language=rust
[4]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[5]: https://www.gnu.org/software/make/
[6]: https://docs.docker.com/engine/install/
[7]: https://www.google.com/chrome/
[8]: https://googlechromelabs.github.io/chrome-for-testing/#stable
