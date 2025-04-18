.PHONY: clean build build-all package package-all publish publish-all

# Variables
VERSION = 0.0.31
NUGET_API_KEY ?= $(NUGET_API_KEY)
MAVEN_AUTH := $(shell echo "$(MAVEN_USERNAME):$(MAVEN_PASSWORD)" | base64)

export CROSS_CONTAINER_ENGINE_NO_BUILDKIT = 1
export LD_LIBRARY_PATH=$(pwd)/target/lib:$LD_LIBRARY_PATH

# Bump version in all relevant files
bump:
	@echo "Bumping version to $(VERSION) recursively..."

	# Update Cargo.toml files (enforce version format X.Y.Z)
	@find crates -name "Cargo.toml" -exec sed -i 's/^version = "[0-9]\+\.[0-9]\+\.[0-9]\+"/version = "$(VERSION)"/' {} \;

	# Update version in lib.rs files (Rust source files)
	@find crates -name "*.rs" -exec sed -i -E "s/(^[[:space:]]*const VERSION: &str = )\"[0-9]+\.[0-9]+\.[0-9]+\";/\1\"$(VERSION)\";/g" {} \;

	# Update version in .csproj files (C# project files)
	@find dotnet -name "*.csproj" -exec sed -i 's/<version>[0-9]\+\.[0-9]\+\.[0-9]\+<\/version>/<version>$(VERSION)<\/version>/' {} \;

	# Update version in JSON files (e.g., package.json)
	@find node -name "package.json" -exec sed -i 's/"version": "[0-9]\+\.[0-9]\+\.[0-9]\+"/"version": "$(VERSION)"/' {} \;

	# Update version in .toml files (e.g., pyproject.toml)
	@find python -name "*.toml" -exec sed -i 's/^version = "[0-9]\+\.[0-9]\+\.[0-9]\+"/version = "$(VERSION)"/' {} \;

	# Update version in Python setup files (setup.py)
	@find python -name "setup.py" -exec sed -i 's/version="[0-9]\+\.[0-9]\+\.[0-9]\+"/version="$(VERSION)"/' {} \;

	# Update version in Markdown files (e.g., README.md)
	@find . -name "*.md" -exec sed -i 's/\b[0-9]\+\.[0-9]\+\.[0-9]\+\b/$(VERSION)/g' {} \;

	# Clean up backup files created by sed
	@find . -name "*.bak" -type f -delete

	# Update version in pom.xml files (C# project files)
	# @find java -name "pom.xml" -exec sed -i 's/<version>[0-9]\+\.[0-9]\+\.[0-9]\+<\/version>/<version>$(VERSION)<\/version>/' {} \;
	@find java -name "pom.xml" -exec sed -i '/<artifactId>client<\/artifactId>/{n;s/<version>[0-9]\+\.[0-9]\+\.[0-9]\+<\/version>/<version>${VERSION}<\/version>/}' {} \;

	# Update version in conanfile.py (conan package manager)
	@find c -name "*.py" -exec sed -i 's/version = "[0-9]\+\.[0-9]\+\.[0-9]\+"/version = "$(VERSION)"/' {} \;

	@echo "Version bump completed to $(VERSION)"

# Clean up
clean:
	rm -rf node/lib node/*.tgz node/*.csv dotnet/lib dotnet/runtime dotnet/*.csv dotnet/bin dotnet/obj
	rm -rf python/openiap/lib python/*.csv python/build python/dist
	rm -rf target/lib target/cli
	rm -rf java/lib java/target

# Create target directories
prepare:
	mkdir -p target/lib target/cli

# Build Rust binaries
build-linux:
	mkdir -p target/lib target/cli
	cross build --target x86_64-unknown-linux-gnu --release
	cp target/x86_64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-x64.so
	cp target/x86_64-unknown-linux-gnu/release/openiap target/cli/linux-x64-openiap
	cross build --target aarch64-unknown-linux-gnu --release
	cp target/aarch64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-arm64.so
	cp target/aarch64-unknown-linux-gnu/release/openiap target/cli/linux-arm64-openiap
	cp crates/clib/clib_openiap.h php/src/clib_openiap.h
	cp crates/clib/clib_openiap.h java/src/main/java/io/openiap/clib_openiap.h
	cp crates/clib/clib_openiap.h c/clib_openiap.h
	cp crates/clib/clib_openiap.h go/clib_openiap.h

build-macos:
	mkdir -p target/lib target/cli
	cross build --target aarch64-apple-darwin --release
	cp target/aarch64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-arm64.dylib
	cp target/aarch64-apple-darwin/release/openiap target/cli/macos-arm64-openiap
	cross build --target x86_64-apple-darwin --release
	cp target/x86_64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-x64.dylib
	cp target/x86_64-apple-darwin/release/openiap target/cli/macos-x64-openiap

build-windows:
	mkdir -p target/lib target/cli
	cross build --target x86_64-pc-windows-gnu --release
	cp target/x86_64-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-x64.dll
	cp target/x86_64-pc-windows-gnu/release/openiap.exe target/cli/windows-x64-openiap.exe
	cross build --target i686-pc-windows-gnu --release
	cp target/i686-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-i686.dll
	cp target/i686-pc-windows-gnu/release/openiap.exe target/cli/windows-i686-openiap.exe

build-java:
	# (cd java && mvn clean package)
build-go:
	(cd go && go build -o cli ./cmd/cli)

copy-lib:
	rm -rf node/lib && mkdir -p node/lib && cp target/lib/* node/lib
	rm -rf dotnet/lib && mkdir -p dotnet/lib && cp target/lib/* dotnet/lib
	rm -rf python/openiap/lib && mkdir -p python/openiap/lib && cp target/lib/* python/openiap/lib
	rm -rf java/lib && mkdir -p java/lib && cp target/lib/* java/lib
	rm -rf c/lib && mkdir -p c/lib && cp target/lib/* c/lib
	rm -rf go/lib && mkdir -p go/lib && cp target/lib/* go/lib

# Package language bindings
package-node:
	@echo "Building Node.js package"
	rm -rf node/lib && mkdir -p node/lib && cp target/lib/* node/lib
	(cd node && npm run build && npm pack)

package-dotnet:
	@echo "Building .NET package"
	rm -rf dotnet/lib && mkdir -p dotnet/lib && cp target/lib/* dotnet/lib
	(cd dotnet && dotnet build --configuration Release openiap.csproj && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release openiap.csproj)
	(cd dotnet && dotnet build --configuration Release openiap-slim.csproj && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release openiap-slim.csproj)

package-python:
	@echo "Building Python package"
	rm -rf python/openiap/lib && mkdir -p python/openiap/lib && cp target/lib/* python/openiap/lib
	(cd python && python setup.py sdist)

package-java:
	@echo "Building java jar"
	rm -rf java/lib && mkdir -p java/lib && cp target/lib/* java/lib
	(cd java && mvn clean package)

package-c:
	conan create c -s os=Linux -s arch=x86_64
	conan create c -s os=Linux -s arch=armv8
	conan create c -s os=Macos -s arch=x86_64
	conan create c -s os=Macos -s arch=armv8
	conan create c -s os=Windows -s arch=x86
	conan create c -s os=Windows -s arch=x86_64
publish-node:
	(cd node && npm publish)

publish-dotnet:
	dotnet nuget push dotnet/bin/Release/openiap.$(VERSION).nupkg --source https://api.nuget.org/v3/index.json --api-key $(NUGET_API_KEY)
	dotnet nuget push dotnet/bin/Release/openiap-slim.$(VERSION).nupkg --source https://api.nuget.org/v3/index.json --api-key $(NUGET_API_KEY)

publish-python:
	(cd python && python3 -m twine upload dist/*)
publish-java:
	# no longer needed, we can simply use mvn
	# (cd java/target/central-publishing && curl --request POST \
	# 	--verbose \
	# 	--header 'Authorization: Bearer $(MAVEN_AUTH)' \
	# 	--form bundle=@central-bundle.zip \
	# 	https://central.sonatype.com/api/v1/publisher/upload)
	(cd java && mvn deploy)

publish-cargo:
	cargo publish -p openiap-proto --allow-dirty
	cargo publish -p openiap-client --allow-dirty
	cargo publish -p openiap --allow-dirty
	cargo publish -p openiap-clib --allow-dirty --no-verify

# Combined tasks
build-all: clean prepare build-linux build-macos build-windows build-java copy-lib
package-all: package-node package-dotnet package-python package-java
publish-all: publish-node publish-dotnet publish-python publish-java publish-cargo

build-and-package-all: build-all package-all
build-and-publish-all: build-all package-all publish-all
latest: build-all package-all publish-all

echo-done:
	@echo "Build and publish process completed!"
