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
    public Pointer roles;
    public int roles_len;

    private transient List<String> roleList;
    public List<String> getRoleList() {
        return roleList;
    }
    
    public User(Pointer p) {
        super(p);
        read();
        readRoles();
    }

    private void readRoles() {
        roleList = new ArrayList<>();
        if (roles != null) {
            for (int i = 0; i < roles_len; i++) {
                Pointer ptr = roles.getPointer(i * Native.POINTER_SIZE);
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