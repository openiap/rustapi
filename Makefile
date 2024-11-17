.PHONY: clean build build-all package package-all publish publish-all

# Variables
VERSION = 0.0.16
NUGET_API_KEY ?= $(NUGET_API_KEY)
export CROSS_CONTAINER_ENGINE_NO_BUILDKIT = 1

# SHELL := /bin/bash
# Clean up
clean:
	rm -rf node/lib node/*.tgz node/*.csv dotnet/lib dotnet/runtime dotnet/*.csv dotnet/bin dotnet/obj
	rm -rf python/openiap/lib python/*.csv python/build python/dist
	rm -rf target/lib target/cli

# Create target directories
prepare:
	mkdir -p target/lib target/cli

# Build Rust binaries
build-linux:
	cross build --target x86_64-unknown-linux-gnu --release
	cp target/x86_64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-x64.so
	cp target/x86_64-unknown-linux-gnu/release/openiap target/cli/linux-x64-openiap
	cross build --target aarch64-unknown-linux-gnu --release
	cp target/aarch64-unknown-linux-gnu/release/libopeniap_clib.so target/lib/libopeniap-linux-arm64.so
	cp target/aarch64-unknown-linux-gnu/release/openiap target/cli/linux-arm64-openiap

build-macos:
	cross build --target aarch64-apple-darwin --release
	cp target/aarch64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-arm64.dylib
	cp target/aarch64-apple-darwin/release/openiap target/cli/macos-arm64-openiap
	cross build --target x86_64-apple-darwin --release
	cp target/x86_64-apple-darwin/release/libopeniap_clib.dylib target/lib/libopeniap-macos-x64.dylib
	cp target/x86_64-apple-darwin/release/openiap target/cli/macos-x64-openiap

build-windows:
	cross build --target x86_64-pc-windows-gnu --release
	cp target/x86_64-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-x64.dll
	cp target/x86_64-pc-windows-gnu/release/openiap.exe target/cli/windows-x64-openiap.exe
	cross build --target i686-pc-windows-gnu --release
	cp target/i686-pc-windows-gnu/release/openiap_clib.dll target/lib/openiap-windows-i686.dll
	cp target/i686-pc-windows-gnu/release/openiap.exe target/cli/windows-i686-openiap.exe

# Package language bindings
package-node:
	echo "Building Node.js package"
	rm -rf node/lib && mkdir -p node/lib && cp target/lib/* node/lib
	(cd node && npm pack)

package-dotnet:
	echo "Building .NET package"
	rm -rf dotnet/lib && mkdir -p dotnet/lib && cp target/lib/* dotnet/lib
	(cd dotnet && dotnet build --configuration Release openiap.csproj && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release openiap.csproj)
	(cd dotnet && dotnet build --configuration Release openiap-slim.csproj && dotnet pack -p:NuspecFile=openiap.nuspec --configuration Release openiap-slim.csproj)

package-python:
	echo "Building Python package"
	rm -rf python/openiap/lib && mkdir -p python/openiap/lib && cp target/lib/* python/openiap/lib
	(cd python && python setup.py sdist)

# Publish language bindings
publish-node:
	(cd node && npm publish)

publish-dotnet:
	dotnet nuget push dotnet/bin/Release/openiap.$(VERSION).nupkg --source https://api.nuget.org/v3/index.json --api-key $(NUGET_API_KEY)
	dotnet nuget push dotnet/bin/Release/openiap-slim.$(VERSION).nupkg --source https://api.nuget.org/v3/index.json --api-key $(NUGET_API_KEY)

publish-python:
	(cd python && python3 -m twine upload dist/*)

publish-cargo:
	cargo publish -p openiap-proto --allow-dirty
	cargo publish -p openiap-client --allow-dirty
	cargo publish -p openiap --allow-dirty
	cargo publish -p openiap-clib --allow-dirty

# Combined tasks
build-all: clean prepare build-linux build-macos build-windows
package-all: package-node package-dotnet package-python
publish-all: publish-node publish-dotnet publish-python publish-cargo

build-and-package-all: build-all package-all
build-and-publish-all: build-all package-all publish-all

echo-done:
	echo "Build and publish process completed!"
