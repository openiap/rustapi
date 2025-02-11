package io.openiap;

import java.io.File;

public class NativeLoader {
    public static String loadLibrary(String libName) {
        String arch = System.getProperty("os.arch");
        String os = System.getProperty("os.name").toLowerCase();
        String libPath = "";

        if (os.contains("win")) {
            libPath = "lib/openiap-windows-" + (arch.contains("64") ? "x64" : "i686") + ".dll";
        } else if (os.contains("mac")) {
            libPath = "lib/libopeniap-macos-" + (arch.contains("64") ? "x64" : "arm64") + ".dylib";
        } else if (os.contains("linux")) {
            libPath = "lib/libopeniap-linux-" + (arch.contains("64") ? "x64" : "arm64") + ".so";
        }

        File localLib = new File(libPath);
        if (localLib.exists()) {
            System.load(localLib.getAbsolutePath());
            System.out.println("Loaded native library from local file: " + libPath);
            return localLib.getAbsolutePath();
        } else {
            throw new RuntimeException("Native library not found: " + libPath);
        }
    }
}
