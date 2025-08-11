package io.openiap;

import java.io.*;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

public class NativeLoader {

    public static String getBootstrapPath() {
        String libfile;
        String arch = System.getProperty("os.arch");
        String os = System.getProperty("os.name").toLowerCase();
        boolean is64 = arch.contains("64");
        boolean dumpLoadingPaths = System.getenv("DEBUG") != null;
        if (dumpLoadingPaths)
            System.out.println("***************");
        if (dumpLoadingPaths)
            System.out.println("Architecture: " + arch);

        if (os.contains("win")) {
            if (dumpLoadingPaths)
                System.out.println("OS: Windows");
            if (is64) {
                libfile = arch.contains("arm") ? "bootstrap-windows-arm64.dll" : "bootstrap-windows-x64.dll";
            } else {
                libfile = "bootstrap-windows-i686.dll";
            }
        } else if (os.contains("linux")) {
            if (dumpLoadingPaths)
                System.out.println("OS: Linux");
            if (!is64)
                throw new RuntimeException("Linux requires a 64-bit process");
            boolean isAlpine = new File("/etc/alpine-release").exists();
            if (isAlpine) {
                libfile = arch.contains("aarch64") || arch.contains("arm64") ? "bootstrap-linux-musl-arm64.so"
                        : "bootstrap-linux-musl-x64.so";
            } else {
                libfile = arch.contains("aarch64") || arch.contains("arm64") ? "bootstrap-linux-arm64.so"
                        : "bootstrap-linux-x64.so";
            }
        } else if (os.contains("mac")) {
            if (dumpLoadingPaths)
                System.out.println("OS: macOS");
            if (!is64)
                throw new RuntimeException("macOS requires a 64-bit process");
            libfile = arch.contains("aarch64") || arch.contains("arm64") ? "bootstrap-macos-arm64.dylib"
                    : "bootstrap-macos-x64.dylib";
        } else {
            throw new RuntimeException("Unsupported OS platform: " + os);
        }

        if (dumpLoadingPaths)
            System.out.println("****************************");
        if (dumpLoadingPaths)
            System.out.println("Loading library " + libfile + " for " + os + " (" + arch + ")");

        // 1. First try to load from JAR resources (production/Maven installation)
        try {
            String resourcePath = "/lib/" + libfile;
            if (dumpLoadingPaths)
                System.out.println("Testing resource path: " + resourcePath);
            
            InputStream resourceStream = NativeLoader.class.getResourceAsStream(resourcePath);
            if (resourceStream != null) {
                if (dumpLoadingPaths)
                    System.out.println("Found bootstrap library in JAR resources");
                
                // Create temp file with proper extension
                String extension = libfile.substring(libfile.lastIndexOf('.'));
                Path tempFile = Files.createTempFile("openiap_bootstrap", extension);
                
                // Copy resource to temp file
                Files.copy(resourceStream, tempFile, StandardCopyOption.REPLACE_EXISTING);
                resourceStream.close();
                
                // Make executable on Unix systems
                if (!os.contains("win")) {
                    tempFile.toFile().setExecutable(true);
                }
                
                String tempPath = tempFile.toAbsolutePath().toString();
                if (dumpLoadingPaths)
                    System.out.println("Extracted bootstrap library to: " + tempPath);
                
                return tempPath;
            }
        } catch (IOException e) {
            if (dumpLoadingPaths)
                System.out.println("Failed to extract from JAR: " + e.getMessage());
        }

        // 2. Fall back to file system search for development environment
        String[] searchDirs = {
                ".", "..", "../..", "../../.."
        };
        String[] targets = { "target/debug", "target/release" };
        String debugLibName, releaseLibName;
        if (os.contains("win")) {
            debugLibName = "openiap_bootstrap.dll";
            releaseLibName = "openiap_bootstrap.dll";
        } else if (os.contains("mac")) {
            debugLibName = "libopeniap_bootstrap.dylib";
            releaseLibName = "libopeniap_bootstrap.dylib";
        } else {
            debugLibName = "libopeniap_bootstrap.so";
            releaseLibName = "libopeniap_bootstrap.so";
        }

        // Search for debug/release builds up to 3 parent directories
        for (String dir : searchDirs) {
            for (String target : targets) {
                File debugLib = new File(dir + "/" + target, debugLibName);
                if (dumpLoadingPaths)
                    System.out.println("Testing libPath " + debugLib.getAbsolutePath());
                if (debugLib.exists())
                    return debugLib.getAbsolutePath();

                File releaseLib = new File(dir + "/" + target, releaseLibName);
                if (dumpLoadingPaths)
                    System.out.println("Testing libPath " + releaseLib.getAbsolutePath());
                if (releaseLib.exists())
                    return releaseLib.getAbsolutePath();
            }
        }

        // Search for runtime/lib folders up to 3 parent directories
        String[] libFolders = { "runtimes", "lib" };
        for (String dir : searchDirs) {
            for (String folder : libFolders) {
                File f = new File(dir + "/" + folder, libfile);
                if (dumpLoadingPaths)
                    System.out.println("Testing libPath " + f.getAbsolutePath());
                if (f.exists())
                    return f.getAbsolutePath();
            }
        }

        // Development environment: ../../../lib
        File devLib = new File("./../../../lib", libfile);
        if (dumpLoadingPaths)
            System.out.println("Testing libPath " + devLib.getAbsolutePath());
        if (devLib.exists())
            return devLib.getAbsolutePath();

        throw new RuntimeException("Library " + libfile + " not found in JAR resources or any known directory.");
    }

    // ...existing code...
    public static String loadLibrary(String dummy) {
        // This method is kept for compatibility with previous versions
        return loadLibrary();
    }
    
    public static String loadLibrary() {
        // 1. Load bootstrap library
        String bootstrapPath = getBootstrapPath();
        System.load(bootstrapPath);

        // 2. Use JNA to call the bootstrap() function
        Bootstrap bootstrap = com.sun.jna.Native.load(bootstrapPath, Bootstrap.class);
        com.sun.jna.Pointer mainLibPathPtr = bootstrap.bootstrap();
        String mainLibPath = mainLibPathPtr.getString(0); // Read C string

        if (mainLibPath == null || mainLibPath.isEmpty()) {
            throw new RuntimeException("Failed to get main library path from bootstrap");
        }
        if (mainLibPath.contains("Error")) {
            throw new RuntimeException("Bootstrap error: " + mainLibPath);
        }

        // 3. Load the main library
        System.load(mainLibPath);

        return mainLibPath;
    }
}