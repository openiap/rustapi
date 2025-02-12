package io.openiap;

import com.sun.jna.FromNativeConverter;
import com.sun.jna.FromNativeContext;
import com.sun.jna.ToNativeConverter;
import com.sun.jna.ToNativeContext;
import com.sun.jna.TypeMapper;

public class BooleanTypeMapper implements TypeMapper {
    private static final FromNativeConverter FROM_NATIVE = new FromNativeConverter() {
        @Override
        public Object fromNative(Object nativeValue, FromNativeContext context) {
            // Native value is a Byte, convert to boolean.
            if (nativeValue instanceof Byte) {
                return ((Byte) nativeValue) != 0;
            }
            return nativeValue;
        }

        @Override
        public Class<?> nativeType() {
            return Byte.class;
        }
    };

    private static final ToNativeConverter TO_NATIVE = new ToNativeConverter() {
        @Override
        public Object toNative(Object value, ToNativeContext context) {
            // Convert boolean to a single byte: 1 for true, 0 for false.
            return (byte) (((Boolean) value) ? 1 : 0);
        }

        @Override
        public Class<?> nativeType() {
            return Byte.class;
        }
    };

    @Override
    public FromNativeConverter getFromNativeConverter(Class<?> javaType) {
        System.out.println("getFromNativeConverter: " + javaType);
        if (javaType == Boolean.class || javaType == boolean.class) {
            return FROM_NATIVE;
        }
        return null;
    }

    @Override
    public ToNativeConverter getToNativeConverter(Class<?> javaType) {
        System.out.println("getToNativeConverter: " + javaType);
        if (javaType == Boolean.class || javaType == boolean.class) {
            return TO_NATIVE;
        }
        return null;
    }
}
