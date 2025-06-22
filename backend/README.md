# Rocket & Sqlx powered backend of `realworld-rocket-react` app

## Spec

Find `RealWorld` API specification [here][1].

The `RealWorld` repository also includes a `Postman` collection that
_should_ be used to run tests on CI with [`Newman`][2].

## Setup

Make sure you got `cargo`, `GNU Make`, and `docker` installed.
Change your working directory to `backend` and hit:

```console
$ make setup
```

You should now be able to start the back-end in watch mode with:

```console
$ make watch
```

<!-- -------------------------------- LINKS -------------------------------- -->

[1]: https://github.com/gothinkster/realworld/blob/09e8fa29ef0ee39fa5d1caecfa0b4e5f090bbe92/api/openapi.yml
[2]: https://learning.postman.com/docs/collections/using-newman-cli/command-line-integration-with-newman/
