package io.openiap;

import com.sun.jna.Library;
import com.sun.jna.Pointer;

public interface ClibOpenIAP extends Library {
    Pointer create_client();
}
