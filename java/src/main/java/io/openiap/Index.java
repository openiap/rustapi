package io.openiap;

import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import com.fasterxml.jackson.databind.JsonNode;

@JsonIgnoreProperties(ignoreUnknown = true)
public class Index {
    public String name;
    public JsonNode key;
    public boolean unique;
    public boolean sparse;
    public boolean background;
    public int expireAfterSeconds;
}
