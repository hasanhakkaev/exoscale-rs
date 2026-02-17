
# Dependencies
.PHONY: install-open-api-generator
install-open-api-generator:
	mkdir -p ~/bin/openapitools
	curl https://raw.githubusercontent.com/OpenAPITools/openapi-generator/master/bin/utils/openapi-generator-cli.sh > ~/bin/openapitools/openapi-generator-cli
	chmod u+x ~/bin/openapitools/openapi-generator-cli
	export PATH=$PATH:~/bin/openapitools/

.PHONY: pull-oapi-spec
pull-oapi-spec:
	@wget -q --show-progress --progress=dot https://openapi-v2.exoscale.com/source.yaml -O- > static/source.yaml

.PHONY: generate
generate: install-open-api-generator pull-oapi-spec
	~/bin/openapitools/openapi-generator-cli generate -i ./static/source.yaml -g rust -o ./ -c config.yaml --skip-validate-spec
	cargo fix --lib -p exoscale-rs --allow-dirty
	rm -rf git_push.sh .travis.yml
	@if [ -f src/models/background__bg__writer_settings.rs ]; then \
		mv src/models/background__bg__writer_settings.rs src/models/background_bg_writer_settings.rs; \
	fi
	@if [ -f src/models/write_ahead_log__wal__settings.rs ]; then \
		mv src/models/write_ahead_log__wal__settings.rs src/models/write_ahead_log_wal_settings.rs; \
	fi
	sed -i.bak 's/background__bg__writer_settings/background_bg_writer_settings/g' src/models/mod.rs && rm -f src/models/mod.rs.bak
	sed -i.bak 's/write_ahead_log__wal__settings/write_ahead_log_wal_settings/g' src/models/mod.rs && rm -f src/models/mod.rs.bak
	# Verify changes
	@if [ -f src/models/background__bg__writer_settings.rs ]; then echo "Error: background__bg__writer_settings.rs still exists"; exit 1; fi
	@if [ ! -f src/models/background_bg_writer_settings.rs ]; then echo "Error: background_bg_writer_settings.rs missing"; exit 1; fi
	@if [ -f src/models/write_ahead_log__wal__settings.rs ]; then echo "Error: write_ahead_log__wal__settings.rs still exists"; exit 1; fi
	@if [ ! -f src/models/write_ahead_log_wal_settings.rs ]; then echo "Error: write_ahead_log_wal_settings.rs missing"; exit 1; fi
	@if grep -q "background__bg__writer_settings" src/models/mod.rs; then echo "Error: mod.rs still contains background__bg__writer_settings"; exit 1; fi
	@if grep -q "write_ahead_log__wal__settings" src/models/mod.rs; then echo "Error: mod.rs still contains write_ahead_log__wal__settings"; exit 1; fi
	cargo check --lib

.PHONY: test
test:
	cargo test

.PHONY: bump-version
bump-version:
ifndef VERSION
	$(error VERSION is required. Usage: make bump-version VERSION=4.0.0)
endif
	@echo "Bumping version to $(VERSION)..."
	sed -i.bak 's/^version = ".*"/version = "$(VERSION)"/' Cargo.toml && rm -f Cargo.toml.bak
	sed -i.bak 's/^packageVersion: .*/packageVersion: $(VERSION)/' config.yaml && rm -f config.yaml.bak
	sed -i.bak 's|exoscale-rs/[0-9]*\.[0-9]*\.[0-9]*|exoscale-rs/$(VERSION)|' src/apis/configuration.rs && rm -f src/apis/configuration.rs.bak
	sed -i.bak 's|exoscale-rs/[0-9]*\.[0-9]*\.[0-9]*|exoscale-rs/$(VERSION)|' static/templates/reqwest/configuration.mustache && rm -f static/templates/reqwest/configuration.mustache.bak
	@echo "Version bumped to $(VERSION) in Cargo.toml, config.yaml, configuration.rs, configuration.mustache"
