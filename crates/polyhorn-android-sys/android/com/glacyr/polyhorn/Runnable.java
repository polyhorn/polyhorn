package com.glacyr.polyhorn;

import android.os.Handler;
import android.os.Looper;

public class Runnable implements java.lang.Runnable {
    private long data;

    private Runnable(long data) {
        this.data = data;
    }

    private void queue() {
        new Handler(Looper.getMainLooper()).post(this);
    }

    public void run() {
        Runnable.main(this.data);
    }

    private static native void main(long data);
}
