package io.openiap;

import com.sun.jna.Structure;
import java.util.Arrays;
import java.util.List;

public class SigninParameters extends Structure {
    public String username;
    public String password;
    public String jwt;
    public String agent;
    public String version;
    public byte longtoken;
    public byte validateonly;
    public byte ping;
    public int request_id;

    public SigninParameters() {
        username = "";
        password = "";
        jwt = "";
        agent = "";
        version = "";
        longtoken = 0;
        validateonly = 0;
        ping = 0;
        request_id = 0;
    }

    @Override
    protected List<String> getFieldOrder() {
        return Arrays.asList(
            "username", "password", "jwt", "agent", "version", "longtoken", "validateonly", "ping", "request_id"
        );
    }

    public static class Builder {
        private SigninParameters instance = new SigninParameters();

        public Builder username(String username) {
            instance.username = username;
            return this;
        }

        public Builder password(String password) {
            instance.password = password;
            return this;
        }

        public Builder jwt(String jwt) {
            instance.jwt = jwt;
            return this;
        }

        public Builder agent(String agent) {
            instance.agent = agent;
            return this;
        }

        public Builder version(String version) {
            instance.version = version;
            return this;
        }

        public Builder longtoken(boolean longtoken) {
            instance.longtoken = (byte) (longtoken ? 1 : 0);
            return this;
        }

        public Builder validateonly(boolean validateonly) {
            instance.validateonly = (byte) (validateonly ? 1 : 0);
            return this;
        }

        public Builder ping(boolean ping) {
            instance.ping = (byte) (ping ? 1 : 0);
            return this;
        }

        public SigninParameters build() {
            instance.write();
            return instance;
        }
    }
}
