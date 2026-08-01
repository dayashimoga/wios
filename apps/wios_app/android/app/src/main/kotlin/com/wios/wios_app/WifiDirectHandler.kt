package com.wios.wios_app

import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.net.wifi.p2p.*
import android.os.Build
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.EventChannel
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel

/**
 * WiFi Direct (Wi-Fi P2P) native Android handler.
 * Provides peer discovery, connection, and data transfer via platform channels.
 */
class WifiDirectHandler(
    private val context: Context,
    flutterEngine: FlutterEngine
) : MethodChannel.MethodCallHandler {

    private val channel = MethodChannel(flutterEngine.dartExecutor.binaryMessenger, "com.wios/wifi_direct")
    private var eventSink: EventChannel.EventSink? = null
    private var manager: WifiP2pManager? = null
    private var wifiChannel: WifiP2pManager.Channel? = null
    private var receiver: BroadcastReceiver? = null

    init {
        channel.setMethodCallHandler(this)
        EventChannel(flutterEngine.dartExecutor.binaryMessenger, "com.wios/wifi_direct/events")
            .setStreamHandler(object : EventChannel.StreamHandler {
                override fun onListen(arguments: Any?, events: EventChannel.EventSink?) {
                    eventSink = events
                }
                override fun onCancel(arguments: Any?) {
                    eventSink = null
                }
            })
    }

    override fun onMethodCall(call: MethodCall, result: MethodChannel.Result) {
        when (call.method) {
            "initialize" -> initialize(result)
            "startDiscovery" -> startDiscovery(result)
            "stopDiscovery" -> stopDiscovery(result)
            "connect" -> {
                val address = call.argument<String>("deviceAddress") ?: ""
                connect(address, result)
            }
            "disconnect" -> disconnect(result)
            "getConnectionInfo" -> getConnectionInfo(result)
            else -> result.notImplemented()
        }
    }

    private fun initialize(result: MethodChannel.Result) {
        manager = context.getSystemService(Context.WIFI_P2P_SERVICE) as? WifiP2pManager
        wifiChannel = manager?.initialize(context, context.mainLooper, null)

        if (manager == null || wifiChannel == null) {
            result.success(false)
            return
        }

        // Register broadcast receiver for Wi-Fi P2P events
        val filter = IntentFilter().apply {
            addAction(WifiP2pManager.WIFI_P2P_STATE_CHANGED_ACTION)
            addAction(WifiP2pManager.WIFI_P2P_PEERS_CHANGED_ACTION)
            addAction(WifiP2pManager.WIFI_P2P_CONNECTION_CHANGED_ACTION)
            addAction(WifiP2pManager.WIFI_P2P_THIS_DEVICE_CHANGED_ACTION)
        }

        receiver = object : BroadcastReceiver() {
            override fun onReceive(ctx: Context, intent: Intent) {
                when (intent.action) {
                    WifiP2pManager.WIFI_P2P_PEERS_CHANGED_ACTION -> {
                        manager?.requestPeers(wifiChannel) { peers ->
                            for (device in peers.deviceList) {
                                eventSink?.success(mapOf(
                                    "type" to "peerFound",
                                    "deviceAddress" to device.deviceAddress,
                                    "deviceName" to device.deviceName,
                                    "isGroupOwner" to device.isGroupOwner,
                                    "status" to device.status
                                ))
                            }
                        }
                    }
                    WifiP2pManager.WIFI_P2P_CONNECTION_CHANGED_ACTION -> {
                        val networkInfo = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                            intent.getParcelableExtra(WifiP2pManager.EXTRA_NETWORK_INFO, android.net.NetworkInfo::class.java)
                        } else {
                            @Suppress("DEPRECATION")
                            intent.getParcelableExtra(WifiP2pManager.EXTRA_NETWORK_INFO)
                        }
                        if (networkInfo?.isConnected == true) {
                            manager?.requestConnectionInfo(wifiChannel) { info ->
                                eventSink?.success(mapOf(
                                    "type" to "connected",
                                    "groupOwnerAddress" to info.groupOwnerAddress?.hostAddress,
                                    "isGroupOwner" to info.isGroupOwner
                                ))
                            }
                        } else {
                            eventSink?.success(mapOf("type" to "disconnected"))
                        }
                    }
                }
            }
        }

        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
            context.registerReceiver(receiver, filter, Context.RECEIVER_NOT_EXPORTED)
        } else {
            context.registerReceiver(receiver, filter)
        }
        result.success(true)
    }

    private fun startDiscovery(result: MethodChannel.Result) {
        manager?.discoverPeers(wifiChannel, object : WifiP2pManager.ActionListener {
            override fun onSuccess() { result.success(true) }
            override fun onFailure(reason: Int) { result.success(false) }
        }) ?: result.success(false)
    }

    private fun stopDiscovery(result: MethodChannel.Result) {
        manager?.stopPeerDiscovery(wifiChannel, object : WifiP2pManager.ActionListener {
            override fun onSuccess() { result.success(true) }
            override fun onFailure(reason: Int) { result.success(false) }
        }) ?: result.success(false)
    }

    private fun connect(deviceAddress: String, result: MethodChannel.Result) {
        val config = WifiP2pConfig().apply {
            this.deviceAddress = deviceAddress
        }
        manager?.connect(wifiChannel, config, object : WifiP2pManager.ActionListener {
            override fun onSuccess() { result.success(true) }
            override fun onFailure(reason: Int) { result.success(false) }
        }) ?: result.success(false)
    }

    private fun disconnect(result: MethodChannel.Result) {
        manager?.removeGroup(wifiChannel, object : WifiP2pManager.ActionListener {
            override fun onSuccess() { result.success(true) }
            override fun onFailure(reason: Int) { result.success(false) }
        }) ?: result.success(false)
    }

    private fun getConnectionInfo(result: MethodChannel.Result) {
        manager?.requestConnectionInfo(wifiChannel) { info ->
            result.success(mapOf(
                "groupOwnerAddress" to info.groupOwnerAddress?.hostAddress,
                "isGroupOwner" to info.isGroupOwner,
                "groupFormed" to info.groupFormed
            ))
        } ?: result.success(null)
    }

    fun dispose() {
        try { context.unregisterReceiver(receiver) } catch (_: Exception) {}
    }
}
