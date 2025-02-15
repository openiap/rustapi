package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import com.sun.jna.Memory;
import com.sun.jna.Native;
import java.util.Arrays;
import java.util.List;
import com.fasterxml.jackson.databind.ObjectMapper;
import java.util.ArrayList;

public class PushWorkitem extends Structure {
    public String wiq;
    public String wiqid;
    public String name;
    public String payload;
    public long nextrun;
    public String success_wiqid;
    public String failed_wiqid;
    public String success_wiq;
    public String failed_wiq;
    public int priority;
    public Pointer files;
    public int files_len;
    public int request_id;

    public PushWorkitem() {
        super();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "wiq", "wiqid", "name", "payload", "nextrun", 
            "success_wiqid", "failed_wiqid", "success_wiq", "failed_wiq",
            "priority", "files", "files_len", "request_id"
        );
    }

    public static class Builder {
        private static final ObjectMapper objectMapper = new ObjectMapper();
        private PushWorkitem instance = new PushWorkitem();
        private List<String> filesList;
        private List<Memory> allocatedMemory = new ArrayList<>();
        private List<WorkitemFileWrapper> wrappers = new ArrayList<>(); // Keep wrappers alive

        public Builder(String wiq) {
            instance.wiq = wiq;
        }

        public Builder wiqid(String wiqid) {
            instance.wiqid = wiqid;
            return this;
        }

        public Builder name(String name) {
            instance.name = name;
            return this;
        }

        public Builder payload(String payload) {
            instance.payload = payload;
            return this;
        }

        public Builder nextrun(long nextrun) {
            instance.nextrun = nextrun;
            return this;
        }

        public Builder priority(int priority) {
            instance.priority = priority;
            return this;
        }

        public Builder successWiq(String wiq) {
            instance.success_wiq = wiq;
            return this;
        }

        public Builder failedWiq(String wiq) {
            instance.failed_wiq = wiq;
            return this;
        }

        public Builder itemFromObject(Object item) {
            try {
                String json = objectMapper.writeValueAsString(item);
                instance.payload = json;
                return this;
            } catch (Exception e) {
                throw new RuntimeException("Failed to serialize object to JSON", e);
            }
        }

        public Builder files(List<String> files) {
            this.filesList = files;
            return this;
        }

        public PushWorkitem build() {
            try {
                if (filesList != null && !filesList.isEmpty()) {
                    instance.files_len = filesList.size();
                    
                    Memory filesArrayPtr = new Memory(Native.POINTER_SIZE * filesList.size());
                    allocatedMemory.add(filesArrayPtr);
                    instance.files = filesArrayPtr;
                    
                    for (int i = 0; i < filesList.size(); i++) {
                        String filePath = filesList.get(i);
                        
                        // Create the wrapper and keep it alive
                        WorkitemFileWrapper fileWrapper = new WorkitemFileWrapper();
                        fileWrapper.filename = filePath; // Let JNA handle string conversion
                        fileWrapper.id = "";
                        fileWrapper.compressed = (byte)0;
                        fileWrapper.write();
                        
                        // Store wrapper to keep it alive
                        wrappers.add(fileWrapper);
                        
                        // Store pointer in array
                        filesArrayPtr.setPointer(i * Native.POINTER_SIZE, fileWrapper.getPointer());
                    }
                }
                
                instance.write();
                
                // Important: Don't clean up yet - let caller handle cleanup
                return instance;
            } catch (Exception e) {
                cleanup();
                throw e;
            }
        }

        public void cleanup() {
            // Clear wrappers first
            for (WorkitemFileWrapper wrapper : wrappers) {
                wrapper.clear();
            }
            wrappers.clear();

            // Then free allocated memory
            for (Memory mem : allocatedMemory) {
                try {
                    if (mem != null) {
                        Native.free(Pointer.nativeValue(mem));
                    }
                } catch (Exception e) {
                    System.err.println("Error freeing memory: " + e.getMessage());
                }
            }
            allocatedMemory.clear();
        }
    }
}
