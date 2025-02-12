package io.openiap;

import com.sun.jna.Pointer;
import com.sun.jna.Structure;
import com.sun.jna.Native;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class User extends Structure {
    public String id;
    public String name;
    public String username;
    public String email;
    // This field is the pointer to the native array of role pointers.
    public Pointer roles;
    // The number of roles stored.
    public int roles_len;

    // A new field to store the roles as a Java list.
    private transient List<String> roleList;
    public List<String> getRoleList() {
        return roleList;
    }
    
    public User(Pointer p) {
        super(p);
        read();      // reads the fields (id, name, etc.) from native memory
        readRoles(); // now read and store the roles into a Java list
    }

    /**
     * Reads the roles from the native memory and stores them in roleList.
     */
    private void readRoles() {
        roleList = new ArrayList<>();
        if (roles != null) {
            // Iterate based on the roles_len field
            for (int i = 0; i < roles_len; i++) {
                // Calculate the pointer for the i-th role pointer
                Pointer ptr = roles.getPointer(i * Native.POINTER_SIZE);
                // Get the Java string from the native pointer.
                String role = ptr.getString(0);
                roleList.add(role);
            }
        }
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("id", "name", "username", "email", "roles", "roles_len");
    }
}