package io.openiap;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;

@JsonIgnoreProperties(ignoreUnknown = true)  // Add this to handle unknown properties
public class Workitem {
    public String id;
    public String name;
    public String payload;
    public int priority;
    public long nextrun;
    public long lastrun;
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
    
    public static class Builder {
        private final Workitem instance = new Workitem();
        
        public Builder name(String name) {
            instance.name = name;
            return this;
        }
        
        public Builder payload(String payload) {
            instance.payload = payload;
            return this;
        }
        
        public Builder priority(int priority) {
            instance.priority = priority;
            return this;
        }
        
        public Builder wiq(String wiq) {
            instance.wiq = wiq;
            return this;
        }
        
        public Workitem build() {
            return instance;
        }
    }
}
