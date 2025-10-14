PROJECT_NAME=realworld-axum-react

POSTGRES_USER=postgres
POSTGRES_PASSWORD=postgres
POSTGRES_HOST=127.0.0.1
POSTGRES_PORT=5432
POSTGRES_DATABASE=${PROJECT_NAME}
POSTGRES_URL=postgresql://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DATABASE}
POSTGRES_CONTAINER_NAME=${PROJECT_NAME}-postgres

APP_PORT=8000

JAEGER_CONTAINER_NAME=${PROJECT_NAME}-jaeger
JAEGER_HOST=127.0.0.1
JAEGER_UI_PORT=8001
JAEGER_OTEL_PORT=4317

# --------------------------- AUXILIARY BINARIES -------------------------------
.PHONY: tools/install
tools/install:
	cargo install cargo-watch
	cargo install bunyan
	cargo install sqlx-cli --features postgres

.PHONY: tools/uninstall
tools/uninstall:
	@cargo uninstall cargo-watch > /dev/null 2>&1 || true
	@cargo uninstall bunyan > /dev/null 2>&1 || true
	@cargo uninstall sqlx-cli > /dev/null 2>&1 || true

# --------------------------- DATABASE COMMANDS --------------------------------
.PHONY: db/start
db/start:
	docker ps -a | grep -i ${POSTGRES_CONTAINER_NAME} && \
	docker start ${POSTGRES_CONTAINER_NAME} || \
	docker run -d \
	-v ${POSTGRES_CONTAINER_NAME}-data:/var/lib/postgresql/data \
	-p ${POSTGRES_HOST}:${POSTGRES_PORT}:5432 \
	-e POSTGRES_USER=${POSTGRES_USER} \
	-e POSTGRES_PASSWORD=${POSTGRES_PASSWORD} \
	-e POSTGRES_DB=${POSTGRES_DATABASE} \
	--name ${POSTGRES_CONTAINER_NAME} \
	postgres:17

.PHONY: db/migrate/add
db/migrate/add:
	sqlx migrate add -rs $(name)

.PHONY: db/migrate/run
db/migrate/run:
	sqlx migrate run --database-url=${POSTGRES_URL}

.PHONY: db/migrate/revert
db/migrate/revert:
	sqlx migrate revert --database-url=${POSTGRES_URL}

.PHONY: db/reset
db/reset:
	sqlx db reset --database-url=${POSTGRES_URL}

.PHONY: db/seed
db/seed:
	docker cp ./migrations/dev/seed.up.sql ${POSTGRES_CONTAINER_NAME}:docker-entrypoint-initdb.d/seed.up.sql
	docker exec ${POSTGRES_CONTAINER_NAME} \
		psql -U ${POSTGRES_USER} -d ${POSTGRES_DATABASE} -f docker-entrypoint-initdb.d/seed.up.sql

.PHONY: db/seed/revert
db/seed/revert:
	docker cp ./migrations/dev/seed.down.sql ${POSTGRES_CONTAINER_NAME}:docker-entrypoint-initdb.d/seed.down.sql
	docker exec ${POSTGRES_CONTAINER_NAME} \
		psql -U ${POSTGRES_USER} -d ${POSTGRES_DATABASE} -f docker-entrypoint-initdb.d/seed.down.sql

.PHONY: db/setup
db/setup:
	while ! (docker exec ${POSTGRES_CONTAINER_NAME} pg_isready); do \
		echo "⏳ Waiting for the database to start accepting connections..."; done
	sqlx db setup --database-url=${POSTGRES_URL}

.PHONY: db/stop
db/stop:
	@docker ps | grep -i ${POSTGRES_CONTAINER_NAME} && \
	docker stop ${POSTGRES_CONTAINER_NAME} || true

.PHONY: db/shell
db/shell:
	docker exec -it ${POSTGRES_CONTAINER_NAME} \
		psql -U ${POSTGRES_USER} -d ${POSTGRES_DATABASE} 

.PHONY: db/purge
db/purge: db/stop
	@docker ps -a | grep -i ${POSTGRES_CONTAINER_NAME} && \
	docker rm ${POSTGRES_CONTAINER_NAME} || true
	@docker volume rm --force ${POSTGRES_CONTAINER_NAME}-data

.PHONY: db/show
db/show: 
	docker exec -it ${POSTGRES_CONTAINER_NAME} \
		psql -U ${POSTGRES_USER} -d ${POSTGRES_DATABASE} \
		-x -c "SELECT * FROM $(table)"

# ---------------------------- UTILITY SERVICES --------------------------------
.PHONY: jaeger/start
jaeger/start:
	docker ps -a | grep -i ${JAEGER_CONTAINER_NAME} && \
	docker start ${JAEGER_CONTAINER_NAME} || \
	docker run -d \
		-p ${JAEGER_HOST}:${JAEGER_UI_PORT}:16686 \
		-p ${JAEGER_HOST}:${JAEGER_OTEL_PORT}:4317 \
		--name ${JAEGER_CONTAINER_NAME} \
		jaegertracing/all-in-one

.PHONY: jaeger/stop
jaeger/stop:
	@docker ps | grep -i ${JAEGER_CONTAINER_NAME} && \
	docker stop ${JAEGER_CONTAINER_NAME} || true

.PHONY: jaeger/purge
jaeger/purge: jaeger/stop
	@docker ps -a | grep -i ${JAEGER_CONTAINER_NAME} && \
	docker rm ${JAEGER_CONTAINER_NAME} || true

# ---------------------------- CLEANUP COMMANDS --------------------------------
.PHONY: teardown
teardown: tools/uninstall db/purge jaeger/purge

# ---------------------------- SETUP COMMANDS ----------------------------------
.PHONY: dotenv
dotenv:
	@echo "🔑 Setting up dotenv file"
	@echo "" >> .env
	@echo "# The content below has been copied from .env.example file" >> .env
	@cat .env.example | tee -a .env

.PHONY: setup
setup: dotenv tools/install jaeger/start db/start db/setup db/seed prepare

# ------------------------- DEVELOPMENT COMMANDS -------------------------------
.PHONY: prepare
prepare:
	cargo sqlx prepare --database-url=${POSTGRES_URL}

.PHONY: test
test:
	SQLX_OFFLINE=1 cargo test $(p) -- --nocapture

.PHONY: test/e2e
test/e2e:
	SQLX_OFFLINE=1 cargo test $(p) --release --test e2e --features=api-test -- --nocapture

.PHONY: test/e2e/browser
test/e2e/browser:
	cd ../frontend && \
		BACKEND_URL=/ \
		REACT_APP_MODE=production \
  	TURNSTILE_SITE_KEY=1x00000000000000000000AA \
		npm run build:prod
	fuser -k 4444/tcp || true
	chromedriver --port=4444 &
	SQLX_OFFLINE=1 cargo test --release --test e2e --features=browser-test -- --nocapture

.PHONY: test/all
test/all: test test/e2e test/e2e/browser

# NB! Application should be launched with `SKIP_CAPTCHA_VERIFICATION`, `SKIP_EMAIL_VERIFICATION`
# and `SKIP_CONTENT_MODERATION` set to `true`. This is something we are not doing (and not going to)
# in production, but required to make the spec tests pass, since email verification and anti-bot 
# verification is not required by spec.
.PHONY: test/e2e/spec
test/e2e/spec:
	git clone https://github.com/gothinkster/realworld.git --branch main --single-branch || true
	cd ./realworld/api && \
		USERNAME=user`date +%s` \
		PASSWORD=password`date +%s` \
		APIURL=http://localhost:${APP_PORT}/api \
			./run-api-tests.sh

.PHONY: watch
watch: db/start prepare
	fuser -k ${APP_PORT}/tcp || true
	cargo watch --clear --exec "run" --no-dot-ignores

.PHONY: otel
watch/otel: jaeger/start
	fuser -k ${APP_PORT}/tcp || true
	RUST_LOG="realworld_axum_react=info,axum=error" \
		OTEL_EXPORTER_OTLP_ENDPOINT=tcp://${JAEGER_HOST}:${JAEGER_OTEL_PORT} \
		cargo watch --clear --exec "run" --no-dot-ignores

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check
check:
	cargo fmt --check
	cargo sqlx prepare --check
	SQLX_OFFLINE=1 cargo clippy --all-features --all-targets
	SQLX_OFFLINE=1 cargo doc --no-deps --all-features

