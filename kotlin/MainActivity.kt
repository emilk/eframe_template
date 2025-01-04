package rs.egui.eframe_template

import androidx.core.view.WindowCompat;
import androidx.core.view.WindowInsetsCompat;
import androidx.core.view.WindowInsetsControllerCompat;

import android.app.NativeActivity;
import android.content.Intent;
import android.os.Bundle;
import android.os.Build.VERSION;
import android.os.Build.VERSION_CODES;
import android.view.View;
import android.view.WindowManager;

public class MainActivity : NativeActivity() {
	companion object {
		init {	
			System.loadLibrary("eframe_template");
		}
	}

	fun hideSystemUI() {
		// This will put the game behind any cutouts and waterfalls on devices which have
		// them, so the corresponding insets will be non-zero.
		if (VERSION.SDK_INT >= VERSION_CODES.P) {
			getWindow().getAttributes().layoutInDisplayCutoutMode = WindowManager.LayoutParams.LAYOUT_IN_DISPLAY_CUTOUT_MODE_ALWAYS;
		}
		// From API 30 onwards, this is the recommended way to hide the system UI, rather than
		// using View.setSystemUiVisibility.
		var decorView = getWindow().getDecorView();
		var controller = WindowCompat.getInsetsController(getWindow(), decorView);
		controller.hide(WindowInsetsCompat.Type.systemBars());
		controller.hide(WindowInsetsCompat.Type.displayCutout());
		controller.setSystemBarsBehavior(WindowInsetsControllerCompat.BEHAVIOR_SHOW_TRANSIENT_BARS_BY_SWIPE);
	}

	override fun onCreate(savedInstanceState : Bundle?) {
		hideSystemUI();
		super.onCreate(savedInstanceState);
	}

	override fun onResume() {
		hideSystemUI();
		super.onResume();
	}
}
