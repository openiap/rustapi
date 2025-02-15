package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;
import com.sun.jna.Native;
import java.util.Arrays;
import java.util.List;
import java.util.ArrayList;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.annotation.JsonIgnore;
import java.util.HashMap;
import java.util.Map;

public class WorkitemWrapper extends Structure {
    public String id;
    public String name;
    public String payload;
    public int priority;
    public long nextrun;
    public long lastrun;
    
    @JsonIgnore
    public Pointer files;
    
    public int files_len;
    public String state;
    public String wiq;
    public String wiqid;
    public int retries;
    public String username;
    public String success_wiqid;
    public String failed_wiqid;
    public String success_wiq;
    public String failed_wiq;
    public String errormessage;
    public String errorsource;
    public String errortype;

    public WorkitemWrapper(Pointer p) {
        super(p);
        read();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "id", "name", "payload", "priority", "nextrun", "lastrun", 
            "files", "files_len", "state", "wiq", "wiqid", "retries", 
            "username", "success_wiqid", "failed_wiqid", "success_wiq", 
            "failed_wiq", "errormessage", "errorsource", "errortype"
        );
    }

    @JsonIgnore
    private List<WorkitemFile> parseFiles() {
        List<WorkitemFile> result = new ArrayList<>();
        if (files != null && files_len > 0) {
            for (int i = 0; i < files_len; i++) {
                Pointer filePtr = files.getPointer(i * Native.POINTER_SIZE);
                if (filePtr != null) {
                    WorkitemFileWrapper fileWrapper = new WorkitemFileWrapper(filePtr);
                    WorkitemFile file = new WorkitemFile();
                    file.filename = fileWrapper.filename;
                    file.id = fileWrapper.id;
                    file.compressed = fileWrapper.compressed != 0;  // Convert byte to boolean
                    result.add(file);
                }
            }
        }
        return result;
    }

    public String toJson() {
        try {
            Map<String, Object> map = new HashMap<>();
            map.put("id", id);
            map.put("name", name);
            map.put("payload", payload);
            map.put("priority", priority);
            map.put("nextrun", nextrun);
            map.put("lastrun", lastrun);
            map.put("state", state);
            map.put("wiq", wiq);
            map.put("wiqid", wiqid);
            map.put("retries", retries);
            map.put("username", username);
            map.put("success_wiqid", success_wiqid);
            map.put("failed_wiqid", failed_wiqid);
            map.put("success_wiq", success_wiq);
            map.put("failed_wiq", failed_wiq);
            map.put("errormessage", errormessage);
            map.put("errorsource", errorsource);
            map.put("errortype", errortype);
            map.put("files", parseFiles());
            return new ObjectMapper().writeValueAsString(map);
        } catch (Exception e) {
            throw new RuntimeException("Failed to convert workitem to JSON", e);
        }
    }
}
