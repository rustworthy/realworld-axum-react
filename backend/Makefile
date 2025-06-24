PROJECT_NAME=realworld-rocket-react

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

.PHONY: db/migrate/new
db/migrate/new:
	sqlx migrate add -rs $(name)

.PHONY: db/migrate/up
db/migrate/up:
	sqlx migrate run --database-url=${POSTGRES_URL}

.PHONY: db/migrate/down
db/migrate/down:
	sqlx migrate revert --database-url=${POSTGRES_URL}

.PHONY: db/reset
db/reset:
	sqlx db reset --database-url=${POSTGRES_URL}

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
setup: dotenv tools/install jaeger/start db/start db/setup prepare

# ------------------------- DEVELOPMENT COMMANDS -------------------------------
.PHONY: prepare
prepare:
	cargo sqlx prepare --database-url=${POSTGRES_URL}

.PHONY: test/api
test/api:
	cargo test --test api

.PHONY: test/spec
test/spec:
	git clone https://github.com/gothinkster/realworld.git --branch main --single-branch || true
	cd ./realworld/api && APIURL=http://localhost:${APP_PORT} ./run-api-tests.sh

.PHONY: watch
watch: db/start prepare
	fuser -k ${APP_PORT}/tcp || true
	cargo watch --clear --exec "run" 

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: check
check:
	cargo fmt --check
	SQLX_OFFLINE=1 cargo clippy
	cargo doc --no-deps --all-features

.PHONY: all
all: setup
	fuser -k ${APP_PORT}/tcp || true
	RUST_LOG="realworld_rocket_react=info,rocket=error" \
	OTEL_EXPORTER_OTLP_ENDPOINT=tcp://${JAEGER_HOST}:${JAEGER_OTEL_PORT} \
	DATABASE_URL=${POSTGRES_URL} \
	MIGRATE=1 \
		cargo run --release