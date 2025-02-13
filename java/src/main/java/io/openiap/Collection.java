package io.openiap;

public class Collection {
    public String name;
    public String type;
    public Object options;
    public CollectionInfo info;
    public CollectionIndex idIndex;

    public static class CollectionInfo {
        public boolean readOnly;
        public String uuid;
    }

    public static class CollectionIndex {
        public int v;
        public IndexKey key;
        public String name;
    }

    public static class IndexKey {
        public int _id;
    }

}