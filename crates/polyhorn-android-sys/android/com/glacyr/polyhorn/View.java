package com.glacyr.polyhorn;

import android.content.Context;
import android.view.Gravity;
import android.view.ViewGroup;
import android.widget.RelativeLayout;

public class View extends RelativeLayout {
    public View(Context context) {
        super(context);

        this.setGravity(Gravity.TOP | Gravity.LEFT);
    }

    public void setFrame(float x, float y, float width, float height) {
        float density = getResources().getDisplayMetrics().density;

        RelativeLayout.LayoutParams params = new RelativeLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT);
        params.leftMargin = (int) (x * density);
        params.topMargin = (int) (y * density);
        params.width = (int) (width * density);
        params.height = (int) (height * density);
        this.setLayoutParams(params);
    }
}
