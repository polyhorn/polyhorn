package com.glacyr.polyhorn;

import android.os.Bundle;
import android.util.DisplayMetrics;

public class Activity extends android.app.Activity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);

        Application.main(this);
    }

    public Rect getBounds() {
        DisplayMetrics metrics = this.getResources().getDisplayMetrics();
        return new Rect(0.0f, 0.0f, metrics.widthPixels / metrics.density, metrics.heightPixels / metrics.density);
    }
}
