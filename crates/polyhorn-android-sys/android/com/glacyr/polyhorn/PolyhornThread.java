package com.glacyr.polyhorn;

import java.lang.Thread;

public class PolyhornThread extends Thread {
    private long data;

    private PolyhornThread(long data) {
        this.data = data;
    }

    public void run() {
        PolyhornThread.main(this.data);
    }

    private static native void main(long data);
}
