import 'dart:async';
import 'package:flutter/services.dart';

/// WiFi Direct service — Android Wi-Fi P2P + iOS Multipeer Connectivity.
/// Uses platform method channels to native Kotlin/Swift implementations.
class WifiDirectService {
  static const _channel = MethodChannel('com.wios/wifi_direct');
  static const _eventChannel = EventChannel('com.wios/wifi_direct/events');

  StreamSubscription? _eventSub;
  final _peersController = StreamController<List<WifiDirectPeer>>.broadcast();
  final _statusController = StreamController<WifiDirectStatus>.broadcast();
  final List<WifiDirectPeer> _peers = [];

  Stream<List<WifiDirectPeer>> get peers => _peersController.stream;
  Stream<WifiDirectStatus> get status => _statusController.stream;
  List<WifiDirectPeer> get discoveredPeers => List.unmodifiable(_peers);

  /// Initialize WiFi Direct / Multipeer subsystem.
  Future<bool> initialize() async {
    try {
      final result = await _channel.invokeMethod<bool>('initialize');
      _listenEvents();
      return result ?? false;
    } on PlatformException catch (e) {
      _statusController.add(WifiDirectStatus(
        state: WifiDirectState.error,
        message: e.message ?? 'Initialization failed',
      ));
      return false;
    }
  }

  /// Start peer discovery.
  Future<void> startDiscovery() async {
    _peers.clear();
    _peersController.add(_peers);
    await _channel.invokeMethod('startDiscovery');
    _statusController.add(WifiDirectStatus(
      state: WifiDirectState.discovering,
      message: 'Scanning for peers...',
    ));
  }

  /// Stop peer discovery.
  Future<void> stopDiscovery() async {
    await _channel.invokeMethod('stopDiscovery');
    _statusController.add(WifiDirectStatus(
      state: WifiDirectState.idle,
      message: 'Discovery stopped',
    ));
  }

  /// Connect to a peer by device address.
  Future<bool> connect(String deviceAddress) async {
    try {
      final result = await _channel.invokeMethod<bool>('connect', {
        'deviceAddress': deviceAddress,
      });
      return result ?? false;
    } on PlatformException {
      return false;
    }
  }

  /// Disconnect from current group.
  Future<void> disconnect() async {
    await _channel.invokeMethod('disconnect');
  }

  /// Send data to connected peer.
  Future<bool> sendData(Uint8List data) async {
    try {
      final result = await _channel.invokeMethod<bool>('sendData', {
        'data': data,
      });
      return result ?? false;
    } on PlatformException {
      return false;
    }
  }

  /// Send a text message to connected peer.
  Future<bool> sendMessage(String message) async {
    return sendData(Uint8List.fromList(message.codeUnits));
  }

  /// Get connection info (group owner IP, etc.).
  Future<Map<String, dynamic>?> getConnectionInfo() async {
    try {
      final result = await _channel.invokeMethod<Map>('getConnectionInfo');
      return result?.cast<String, dynamic>();
    } on PlatformException {
      return null;
    }
  }

  void _listenEvents() {
    _eventSub = _eventChannel.receiveBroadcastStream().listen((event) {
      if (event is Map) {
        final type = event['type'] as String?;
        switch (type) {
          case 'peerFound':
            final peer = WifiDirectPeer(
              deviceAddress: event['deviceAddress'] as String,
              deviceName: event['deviceName'] as String? ?? 'Unknown',
              isGroupOwner: event['isGroupOwner'] as bool? ?? false,
              status: event['status'] as int? ?? 0,
            );
            _peers.removeWhere((p) => p.deviceAddress == peer.deviceAddress);
            _peers.add(peer);
            _peersController.add(List.from(_peers));
            break;
          case 'peerLost':
            final addr = event['deviceAddress'] as String;
            _peers.removeWhere((p) => p.deviceAddress == addr);
            _peersController.add(List.from(_peers));
            break;
          case 'connected':
            _statusController.add(WifiDirectStatus(
              state: WifiDirectState.connected,
              message: 'Connected to ${event['deviceName']}',
              groupOwnerAddress: event['groupOwnerAddress'] as String?,
            ));
            break;
          case 'disconnected':
            _statusController.add(WifiDirectStatus(
              state: WifiDirectState.idle,
              message: 'Disconnected',
            ));
            break;
          case 'dataReceived':
            // Forward to mesh network layer
            break;
        }
      }
    });
  }

  void dispose() {
    _eventSub?.cancel();
    _peersController.close();
    _statusController.close();
  }
}

/// A discovered WiFi Direct / Multipeer peer.
class WifiDirectPeer {
  final String deviceAddress;
  final String deviceName;
  final bool isGroupOwner;
  final int status;

  WifiDirectPeer({
    required this.deviceAddress,
    required this.deviceName,
    required this.isGroupOwner,
    required this.status,
  });

  String get statusLabel {
    switch (status) {
      case 0: return 'Connected';
      case 1: return 'Invited';
      case 2: return 'Failed';
      case 3: return 'Available';
      case 4: return 'Unavailable';
      default: return 'Unknown';
    }
  }
}

/// WiFi Direct connection state.
enum WifiDirectState { idle, discovering, connecting, connected, error }

/// WiFi Direct status update.
class WifiDirectStatus {
  final WifiDirectState state;
  final String message;
  final String? groupOwnerAddress;

  WifiDirectStatus({
    required this.state,
    required this.message,
    this.groupOwnerAddress,
  });
}
