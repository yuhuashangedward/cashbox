package info.scry.wallet;

import android.net.Uri;
import android.os.Build;
import android.os.Bundle;
import android.content.Intent;

import io.flutter.app.FlutterActivity;
import io.flutter.plugin.common.EventChannel;
import io.flutter.plugins.GeneratedPluginRegistrant;
import io.flutter.plugin.common.MethodCall;
import io.flutter.plugin.common.MethodChannel;
import io.flutter.plugin.common.MethodChannel.MethodCallHandler;
import io.flutter.plugin.common.MethodChannel.Result;

import android.util.Log;

import com.yzq.zxinglibrary.android.CaptureActivity;
import com.yzq.zxinglibrary.common.Constant;

import java.io.IOException;

public class MainActivity extends FlutterActivity {

    private static final String QR_SCAN_CHANNEL = "qr_scan_channel";
    private static final String FILE_SYSTEM_CHANNEL = "file_system_channel";
    private static final int REQUEST_CODE_QR_SCAN = 0;
    private static final int REQUEST_CODE_FILE_SYSTEM = 1;
    private Result mFlutterChannelResult = null;
    private final String QR_SCAN_METHOD = "qr_scan_method";
    private final String FILE_SYSTEM_METHOD = "file_system_method";
    private final String CHARGING_CHANNEL = "samples.flutter.io/charging";
    private final String FLUTTER_LOG_CHANNEL = "android_log";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.LOLLIPOP) {//API>21,设置状态栏颜色透明
            getWindow().setStatusBarColor(0);
        }
        GeneratedPluginRegistrant.registerWith(this);

        new MethodChannel(getFlutterView(), QR_SCAN_CHANNEL)
                .setMethodCallHandler(
                        new MethodCallHandler() {
                            @Override
                            public void onMethodCall(MethodCall call, Result result) {
                                if (call.method.toString().equals(QR_SCAN_METHOD)) {
                                    mFlutterChannelResult = result;
                                    Intent intent = new Intent(MainActivity.this, CaptureActivity.class);
                                    startActivityForResult(intent, REQUEST_CODE_QR_SCAN);
                                }
                            }
                        }
                );
        new MethodChannel(getFlutterView(), FILE_SYSTEM_CHANNEL)
                .setMethodCallHandler(
                        new MethodCallHandler() {
                            @Override
                            public void onMethodCall(MethodCall call, Result result) {
                                if (call.method.toString().equals(FILE_SYSTEM_METHOD)) {
                                    mFlutterChannelResult = result;
                                    Log.d("MainActivity", "begin to call getFileSystem=================>");
                                    Intent intent = new Intent(Intent.ACTION_GET_CONTENT);
                                    intent.setType("*/*");
                                    intent.addCategory(Intent.CATEGORY_OPENABLE);
                                    startActivityForResult(intent, REQUEST_CODE_FILE_SYSTEM);
                                }
                            }
                        }
                );
        //flutter处 log日志保存
        new MethodChannel(getFlutterView(), FLUTTER_LOG_CHANNEL)
                .setMethodCallHandler(
                        new MethodCallHandler() {
                            @Override
                            public void onMethodCall(MethodCall call, Result result) {
                                logPrint(call);
                            }
                        }
                );

        // TODO: 2019/8/19 parker     EventChannel test code
        //  native端，主动通知到 flutter
        new EventChannel(getFlutterView(), CHARGING_CHANNEL).setStreamHandler(
                new EventChannel.StreamHandler() {
                    @Override
                    public void onListen(Object o, EventChannel.EventSink eventSink) {
                        //todo something
                        eventSink.success(o);
                        eventSink.error("error", "something is error", o);
                        // errors参数：
                        // this.code,
                        // this.message,
                        // this.details
                    }

                    @Override
                    public void onCancel(Object o) {

                    }
                }
        );
    }

    private void logPrint(MethodCall call) {
        String tag = call.argument("tag");
        String message = call.argument("msg");
        switch (call.method) {
            case "logV":
                Log.v(tag, message);
                break;
            case "logD":
                Log.d(tag, message);
                break;
            case "logI":
                Log.i(tag, message);
                break;
            case "logW":
                Log.w(tag, message);
                break;
            case "logE":
                Log.e(tag, message);
                break;
        }
    }

    @Override
    protected void onActivityResult(int requestCode, int resultCode, Intent data) {
        super.onActivityResult(requestCode, resultCode, data);
        // 扫描二维码/条码回传
        if (requestCode == REQUEST_CODE_QR_SCAN) {
            if (data != null && resultCode == RESULT_OK) {
                String scanResultString = data.getStringExtra(Constant.CODED_CONTENT);
                if (scanResultString == null) {
                    scanResultString = "";
                }
                mFlutterChannelResult.success(scanResultString);
            } else {
                mFlutterChannelResult.error("resultCode is ===>", "" + resultCode, "");
            }
        }else if(requestCode == REQUEST_CODE_FILE_SYSTEM){
            if (data != null && resultCode == RESULT_OK) {
                //1、在原生应用处获得目标filepath，将地址传给flutter层
                //2、由flutter层，再根据filepath
                Uri uri = data.getData();
                Log.d("MainActivity", "result getFileSystem is=================>"+uri.toString());
                mFlutterChannelResult.success(uri.toString());
            }else{
                mFlutterChannelResult.error("resultCode is ===>", "" + resultCode, "");
            }
        } else {
            Log.d("MainActivity", "unknown method result,requestCode is===>" + requestCode);
        }
    }
}
