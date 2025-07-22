# Axum & Sqlx powered backend of `realworld-axum-react` app

## Spec

Find `RealWorld` API specification [here][1].

The `RealWorld` repository also includes a `Postman` collection that
_should_ be used to run tests on CI with [`Newman`][2].

## Setup

Make sure you got [`cargo`][4], [`GNU Make`][5], and [`docker`][6] installed.
Change your working directory to `backend` and hit:

```console
$ make setup
```

You should now be able to start the back-end in watch mode with:

```console
$ make watch
```

Run end-to-end tests with:

```console
$ make test/e2e
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
$ make test/e2e/browser
$ HEADLESS=1 make test/e2e/browser # to run in headless mode
```

<!-- -------------------------------- LINKS -------------------------------- -->

[1]: https://github.com/gothinkster/realworld/blob/09e8fa29ef0ee39fa5d1caecfa0b4e5f090bbe92/api/openapi.yml
[2]: https://learning.postman.com/docs/collections/using-newman-cli/command-line-integration-with-newman/
[3]: https://testcontainers.com/?language=rust
[4]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[5]: https://www.gnu.org/software/make/
[6]: https://docs.docker.com/engine/install/
[7]: https://www.google.com/chrome/
[8]: https://googlechromelabs.github.io/chrome-for-testing/#stable
