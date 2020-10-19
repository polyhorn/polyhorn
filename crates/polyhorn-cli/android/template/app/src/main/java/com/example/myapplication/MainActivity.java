package { spec.app.android.package };

import android.os.Bundle;
import android.util.DisplayMetrics;
import android.view.Menu;
import android.view.MenuItem;
import androidx.appcompat.app.AppCompatActivity;
import com.glacyr.polyhorn.Application;
import com.glacyr.polyhorn.Rect;
import com.glacyr.polyhorn.View;

public class MainActivity extends AppCompatActivity \{
    public Rect getBounds() \{
        DisplayMetrics metrics = this.getResources().getDisplayMetrics();
        return new Rect(0.0f, 0.0f, metrics.widthPixels / metrics.density, metrics.heightPixels / metrics.density);
    }

    @Override
    protected void onCreate(Bundle savedInstanceState) \{
        super.onCreate(savedInstanceState);

        System.loadLibrary("{ spec.app.android.library }");

        Application.main(this);
    }

    @Override
    public boolean onCreateOptionsMenu(Menu menu) \{
        // Inflate the menu; this adds items to the action bar if it is present.
        getMenuInflater().inflate(R.menu.menu_main, menu);
        return true;
    }

    @Override
    public boolean onOptionsItemSelected(MenuItem item) \{
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        int id = item.getItemId();

        //noinspection SimplifiableIfStatement
        if (id == R.id.action_settings) \{
            return true;
        }

        return super.onOptionsItemSelected(item);
    }
}
