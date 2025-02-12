package io.openiap;

import com.sun.jna.Pointer;
import com.sun.jna.Structure;

import java.util.Arrays;
import java.util.List;

public class User extends Structure {
    public String id;
    public String name;
    public String username;
    public String email;
    public Pointer roles;

    public User(Pointer p) {
        super(p);
        read();
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList("id", "name", "username", "email", "roles");
    }
}
