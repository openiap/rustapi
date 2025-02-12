package io.openiap;

import java.io.File;

public class NativeLoader {
    public static String loadLibrary(String libName) {
        String arch = System.getProperty("os.arch");
        String os = System.getProperty("os.name").toLowerCase();
        String libPath = "";
        // System.out.println("Current path" + System.getProperty("user.dir"));
        String base = System.getProperty("user.dir").endsWith("java") ? "lib/" : "java/lib/";

        File localDir1 = new File("target/debug");
        File localDir2 = new File("../target/debug");
        if(localDir1.exists()) {
            base = "target/debug/";
            if (os.contains("win")) {
                libPath = base + "libopeniap_clib.dll";
            } else if (os.contains("mac")) {
                libPath = base + "libopeniap_clib.dylib";
            } else if (os.contains("linux")) {
                libPath = base + "libopeniap_clib.so";
            }
        } else if(localDir2.exists()) {
            base = "../target/debug/";
            if (os.contains("win")) {
                libPath = base + "libopeniap_clib.dll";
            } else if (os.contains("mac")) {
                libPath = base + "libopeniap_clib.dylib";
            } else if (os.contains("linux")) {
                libPath = base + "libopeniap_clib.so";
            }
        } else if (os.contains("win")) {
            libPath = base + "openiap-windows-" + (arch.contains("64") ? "x64" : "i686") + ".dll";
        } else if (os.contains("mac")) {
            libPath = base + "libopeniap-macos-" + (arch.contains("64") ? "x64" : "arm64") + ".dylib";
        } else if (os.contains("linux")) {
            libPath = base + "libopeniap-linux-" + (arch.contains("64") ? "x64" : "arm64") + ".so";
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
