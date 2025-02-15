package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Memory;
import com.sun.jna.Native;
import com.sun.jna.Pointer;
import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;

public class UpdateWorkitem extends Structure {
    public Pointer workitem;
    public boolean ignoremaxretries;
    public Pointer files;
    public int files_len;
    public int request_id;

    public UpdateWorkitem() {
        super();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("workitem", "ignoremaxretries", "files", "files_len", "request_id");
    }

    public static class Builder {
        private UpdateWorkitem instance = new UpdateWorkitem();
        private List<String> filesList;
        private List<Memory> allocatedMemory = new ArrayList<>();
        private List<WorkitemFileWrapper> wrappers = new ArrayList<>();

        public Builder(Workitem workitem) {
            WorkitemWrapper wrapper = new WorkitemWrapper();
            // Copy fields from workitem to wrapper
            wrapper.id = workitem.id;
            wrapper.name = workitem.name;
            wrapper.payload = workitem.payload;
            wrapper.priority = workitem.priority;
            wrapper.state = workitem.state;
            wrapper.wiq = workitem.wiq;
            wrapper.wiqid = workitem.wiqid;
            wrapper.username = workitem.username;
            wrapper.write();
            instance.workitem = wrapper.getPointer();
        }

        public Builder ignoremaxretries(boolean ignore) {
            instance.ignoremaxretries = ignore;
            return this;
        }

        public Builder files(List<String> files) {
            this.filesList = files;
            return this;
        }

        public UpdateWorkitem build() {
            try {
                if (filesList != null && !filesList.isEmpty()) {
                    instance.files_len = filesList.size();
                    
                    Memory filesArrayPtr = new Memory(Native.POINTER_SIZE * filesList.size());
                    allocatedMemory.add(filesArrayPtr);
                    instance.files = filesArrayPtr;
                    
                    for (int i = 0; i < filesList.size(); i++) {
                        String filePath = filesList.get(i);
                        
                        WorkitemFileWrapper fileWrapper = new WorkitemFileWrapper();
                        fileWrapper.filename = filePath;
                        fileWrapper.id = "";
                        fileWrapper.compressed = (byte)0;
                        fileWrapper.write();
                        
                        wrappers.add(fileWrapper);
                        filesArrayPtr.setPointer(i * Native.POINTER_SIZE, fileWrapper.getPointer());
                    }
                }
                
                instance.write();
                return instance;
            } catch (Exception e) {
                cleanup();
                throw e;
            }
        }

        public void cleanup() {
            for (WorkitemFileWrapper wrapper : wrappers) {
                wrapper.clear();
            }
            wrappers.clear();

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
