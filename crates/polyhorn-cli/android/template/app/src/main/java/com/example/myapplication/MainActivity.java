package { spec.app.android.package };

import android.os.Bundle;
import com.glacyr.polyhorn.Activity;

public class MainActivity extends Activity \{
    @Override
    protected void onCreate(Bundle savedInstanceState) \{
        System.loadLibrary("{ spec.app.android.library }");
        
        super.onCreate(savedInstanceState);
    }
}
