package io.openiap;

import java.io.File;

public class NativeLoader {
    public static String loadLibrary(String libName) {
        String arch = System.getProperty("os.arch");
        String os = System.getProperty("os.name").toLowerCase();
        String libPath = "";
        // System.out.println("Current path" + System.getProperty("user.dir"));
        // if System.getProperty("user.dir") does not ends with "java" then we are in the root directory, then add "java" to the path
        String base = System.getProperty("user.dir").endsWith("java") ? "" : "java/";

        if (os.contains("win")) {
            libPath = base + "lib/openiap-windows-" + (arch.contains("64") ? "x64" : "i686") + ".dll";
        } else if (os.contains("mac")) {
            libPath = base + "lib/libopeniap-macos-" + (arch.contains("64") ? "x64" : "arm64") + ".dylib";
        } else if (os.contains("linux")) {
            libPath = base + "lib/libopeniap-linux-" + (arch.contains("64") ? "x64" : "arm64") + ".so";
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
