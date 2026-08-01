import 'dart:async';
import 'package:flutter/services.dart';

/// UWB ranging service — Android UWB API + iOS Nearby Interaction.
/// Uses platform method channels to native Kotlin/Swift implementations.
/// Supported: iPhone 11+ (U1 chip), Pixel 6+, Samsung S21+ (UWB chipset).
class UwbService {
  static const _channel = MethodChannel('com.wios/uwb');
  static const _eventChannel = EventChannel('com.wios/uwb/events');

  StreamSubscription? _eventSub;
  final _rangeController = StreamController<UwbRangeResult>.broadcast();
  final _statusController = StreamController<UwbStatus>.broadcast();
  bool _isSupported = false;

  Stream<UwbRangeResult> get rangeUpdates => _rangeController.stream;
  Stream<UwbStatus> get status => _statusController.stream;
  bool get isSupported => _isSupported;

  /// Check if device has UWB hardware.
  Future<bool> checkSupport() async {
    try {
      _isSupported = await _channel.invokeMethod<bool>('isSupported') ?? false;
      return _isSupported;
    } on PlatformException {
      _isSupported = false;
      return false;
    }
  }

  /// Start a ranging session with a peer.
  Future<bool> startRanging({
    required String peerId,
    required List<int> peerToken,
  }) async {
    if (!_isSupported) return false;
    try {
      final result = await _channel.invokeMethod<bool>('startRanging', {
        'peerId': peerId,
        'peerToken': Uint8List.fromList(peerToken),
      });
      _listenEvents();
      _statusController.add(UwbStatus(
        state: UwbState.ranging,
        message: 'Ranging with $peerId',
      ));
      return result ?? false;
    } on PlatformException catch (e) {
      _statusController.add(UwbStatus(
        state: UwbState.error,
        message: e.message ?? 'Failed to start ranging',
      ));
      return false;
    }
  }

  /// Stop the current ranging session.
  Future<void> stopRanging() async {
    await _channel.invokeMethod('stopRanging');
    _statusController.add(UwbStatus(
      state: UwbState.idle,
      message: 'Ranging stopped',
    ));
  }

  /// Get the local UWB address/token for sharing with peers.
  Future<List<int>?> getLocalToken() async {
    try {
      final result = await _channel.invokeMethod<Uint8List>('getLocalToken');
      return result?.toList();
    } on PlatformException {
      return null;
    }
  }

  /// Exchange tokens with a peer (via BLE/WiFi Direct) then start ranging.
  Future<bool> rangeWithPeer(String peerId, List<int> peerToken) async {
    return startRanging(peerId: peerId, peerToken: peerToken);
  }

  void _listenEvents() {
    _eventSub?.cancel();
    _eventSub = _eventChannel.receiveBroadcastStream().listen((event) {
      if (event is Map) {
        final type = event['type'] as String?;
        switch (type) {
          case 'rangeResult':
            _rangeController.add(UwbRangeResult(
              peerId: event['peerId'] as String,
              distance: (event['distance'] as num).toDouble(),
              azimuth: (event['azimuth'] as num?)?.toDouble(),
              elevation: (event['elevation'] as num?)?.toDouble(),
              confidence: (event['confidence'] as num?)?.toDouble() ?? 1.0,
              timestamp: DateTime.now(),
            ));
            break;
          case 'peerLost':
            _statusController.add(UwbStatus(
              state: UwbState.peerLost,
              message: 'Peer ${event['peerId']} out of range',
            ));
            break;
          case 'error':
            _statusController.add(UwbStatus(
              state: UwbState.error,
              message: event['message'] as String? ?? 'UWB error',
            ));
            break;
        }
      }
    });
  }

  void dispose() {
    _eventSub?.cancel();
    _rangeController.close();
    _statusController.close();
  }
}

/// A UWB ranging measurement result.
class UwbRangeResult {
  final String peerId;
  /// Distance in meters.
  final double distance;
  /// Horizontal angle in degrees (-180 to 180). Null if not supported.
  final double? azimuth;
  /// Vertical angle in degrees (-90 to 90). Null if not supported.
  final double? elevation;
  /// Confidence (0.0 to 1.0).
  final double confidence;
  final DateTime timestamp;

  UwbRangeResult({
    required this.peerId,
    required this.distance,
    this.azimuth,
    this.elevation,
    required this.confidence,
    required this.timestamp,
  });

  String get distanceLabel {
    if (distance < 1.0) return '${(distance * 100).toStringAsFixed(0)} cm';
    return '${distance.toStringAsFixed(1)} m';
  }

  String get directionLabel {
    if (azimuth == null) return 'Unknown';
    final a = azimuth!;
    if (a > -22.5 && a <= 22.5) return 'Ahead';
    if (a > 22.5 && a <= 67.5) return 'Front-Right';
    if (a > 67.5 && a <= 112.5) return 'Right';
    if (a > 112.5 && a <= 157.5) return 'Behind-Right';
    if (a > -67.5 && a <= -22.5) return 'Front-Left';
    if (a > -112.5 && a <= -67.5) return 'Left';
    if (a > -157.5 && a <= -112.5) return 'Behind-Left';
    return 'Behind';
  }
}

enum UwbState { idle, ranging, peerLost, error }

class UwbStatus {
  final UwbState state;
  final String message;

  UwbStatus({required this.state, required this.message});
}
