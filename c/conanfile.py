from conan import ConanFile
from conan.tools.files import copy
import os

class OpenIAPConan(ConanFile):
    name = "openiap"
    version = "0.0.31"
    license = "MPL-2.0"
    url = "https://github.com/openiap/rustapi"
    homepage = "https://openiap.io"
    description = "C Client library for OpenCore"
    settings = "os", "arch", "compiler", "build_type"
    exports_sources = "include/*", "lib/*", "test_package/*"
    no_copy_source = False

    def build(self):
        if self.settings.os == "Linux":
            if self.settings.arch == "x86_64":
                self.run("gcc test_package/main.c -Llib -lopeniap-linux-x64 -Wl,-rpath=lib -o client_cli")
            elif self.settings.arch == "armv8":
                self.run("gcc test_package/main.c -Llib -lopeniap-linux-arm64 -Wl,-rpath=lib -o client_cli")
        elif self.settings.os == "Macos":
            if self.settings.arch == "x86_64":
                self.run("gcc test_package/main.c -Llib -lopeniap-macos-x64 -Wl,-rpath=lib -o client_cli")
            elif self.settings.arch == "armv8":
                self.run("gcc test_package/main.c -Llib -lopeniap-macos-arm64 -Wl,-rpath=lib -o client_cli")
        elif self.settings.os == "Windows":
            if self.settings.arch == "x86_64":
                self.run("gcc test_package/main.c -Llib -lopeniap-windows-x64 -o client_cli")
            elif self.settings.arch == "x86":
                self.run("gcc test_package/main.c -Llib -lopeniap-windows-i686 -o client_cli")

    def package(self):
        copy(self, "*.h", src="include", dst=os.path.join(self.package_folder, "include"))
        if self.settings.os == "Linux":
            if self.settings.arch == "x86_64":
                copy(self, "libopeniap-linux-x64.so", src="lib", dst=os.path.join(self.package_folder, "lib"))
            elif self.settings.arch == "armv8":
                copy(self, "libopeniap-linux-arm64.so", src="lib", dst=os.path.join(self.package_folder, "lib"))
        elif self.settings.os == "Macos":
            if self.settings.arch == "x86_64":
                copy(self, "libopeniap-macos-x64.dylib", src="lib", dst=os.path.join(self.package_folder, "lib"))
            elif self.settings.arch == "armv8":
                copy(self, "libopeniap-macos-arm64.dylib", src="lib", dst=os.path.join(self.package_folder, "lib"))
        elif self.settings.os == "Windows":
            if self.settings.arch == "x86_64":
                copy(self, "openiap-windows-x64.dll", src="lib", dst=os.path.join(self.package_folder, "bin"))
            elif self.settings.arch == "x86":
                copy(self, "openiap-windows-i686.dll", src="lib", dst=os.path.join(self.package_folder, "bin"))

    def package_info(self):
        self.cpp_info.libs = ["openiap"]
