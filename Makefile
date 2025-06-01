
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
	~/bin/openapitools/openapi-generator-cli generate -i ./static/source.yaml -g rust -o ./ -c config.yaml
	cargo fix --lib -p exoscale-rs --allow-dirty
	rm -rf git_push.sh .travis.yml

.PHONY: test
test:
	cargo test
