package io.openiap;

import java.io.*;

public class NativeLoader {
    public static String loadLibrary(String libName) {
        String arch = System.getProperty("os.arch");
        String os = System.getProperty("os.name").toLowerCase();
        String libPath = "";
        String debugfilename = "libopeniap_clib.so";
        String releasefilename = "libopeniap-linux-" + (arch.contains("64") ? "x64" : "arm64") + ".so";
        if (os.contains("win")) {
            releasefilename = "openiap-windows-" + (arch.contains("64") ? "x64" : "i686") + ".dll";
            debugfilename = "target/debug/libopeniap_clib.dll";
        } else if (os.contains("mac")) {
            releasefilename = "libopeniap-macos-" + (arch.contains("64") ? "x64" : "arm64") + ".dylib";
            debugfilename = "target/debug/libopeniap_clib.dylib";
        }

        
        
        // First, try to load from a development/local environment
        File localDir1 = new File("target/debug/" + debugfilename);
        File localDir2 = new File("../target/debug/" + debugfilename);
        File localDir3 = new File("lib/" + releasefilename);
        File localDir4 = new File("java/lib/" + releasefilename);
        if (localDir1.exists()) {
            libPath = "target/debug/" + debugfilename;
        } else if (localDir2.exists()) {
            libPath = "../target/debug/" + debugfilename;
        } else if (localDir3.exists()) {
            libPath = "lib/" + releasefilename;
        } else if (localDir4.exists()) {
            libPath = "java/lib/" + releasefilename;
        } else {
            libPath = "lib/" + releasefilename;
        }

        // First, try loading from the filesystem.
        File localLib = new File(libPath);
        if (localLib.exists()) {
            System.load(localLib.getAbsolutePath());
            // System.out.println("Loaded native library from file: " + localLib.getAbsolutePath());
            return localLib.getAbsolutePath();
        }

        // If not found on disk, try loading as a resource from the jar.
        try (InputStream in = NativeLoader.class.getClassLoader().getResourceAsStream(libPath)) {
            if (in == null) {
                throw new RuntimeException("Native library not found as resource: " + libPath);
            }
            // Create a temporary file with the correct extension.
            String suffix = libPath.substring(libPath.lastIndexOf('.'));
            File temp = File.createTempFile("openiap-", suffix);
            temp.deleteOnExit();
            try (OutputStream out = new FileOutputStream(temp)) {
                byte[] buffer = new byte[4096];
                int read;
                while ((read = in.read(buffer)) != -1) {
                    out.write(buffer, 0, read);
                }
            }
            System.load(temp.getAbsolutePath());
            // System.out.println("Loaded native library from extracted resource: " + temp.getAbsolutePath());
            return temp.getAbsolutePath();
        } catch (IOException e) {
            throw new RuntimeException("Failed to load native library from resource: " + libPath, e);
        }
    }
}
