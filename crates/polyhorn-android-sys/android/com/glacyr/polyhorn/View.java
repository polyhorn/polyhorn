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

    public void setFrame(Rect frame) {
        float density = getResources().getDisplayMetrics().density;

        RelativeLayout.LayoutParams params = new RelativeLayout.LayoutParams(ViewGroup.LayoutParams.WRAP_CONTENT, ViewGroup.LayoutParams.WRAP_CONTENT);
        params.leftMargin = (int) (frame.x * density);
        params.topMargin = (int) (frame.y * density);
        params.width = (int) (frame.width * density);
        params.height = (int) (frame.height * density);
        this.setLayoutParams(params);
    }

    public Rect getBounds() {
        float density = getResources().getDisplayMetrics().density;
        float width = ((float) this.getWidth()) / density;
        float height = ((float) this.getHeight()) / density;

        return new Rect(0.0f, 0.0f, width, height);
    }
}
