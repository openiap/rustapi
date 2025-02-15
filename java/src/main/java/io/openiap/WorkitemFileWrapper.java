package io.openiap;

import com.sun.jna.Structure;
import com.sun.jna.Pointer;

@Structure.FieldOrder({"filename", "id", "compressed"})
public class WorkitemFileWrapper extends Structure {
    public String filename;
    public String id;
    public byte compressed;

    public WorkitemFileWrapper() {
        super();
        setAutoWrite(false);  // Prevent automatic writes
    }

    public WorkitemFileWrapper(Pointer p) {
        super(p);
        setAutoWrite(false);  // Prevent automatic writes
        read();
    }
}
