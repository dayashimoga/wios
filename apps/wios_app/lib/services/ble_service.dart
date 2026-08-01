import 'dart:async';
import 'package:flutter_blue_plus/flutter_blue_plus.dart';
import 'package:permission_handler/permission_handler.dart';

/// BLE service — scans for nearby BLE devices, connects, and exchanges data.
/// Wires into the Rust TransportProvider trait via flutter_rust_bridge.
class BleService {
  StreamSubscription<List<ScanResult>>? _scanSub;
  final List<BleDevice> _discovered = [];
  final _devicesController = StreamController<List<BleDevice>>.broadcast();

  Stream<List<BleDevice>> get devices => _devicesController.stream;
  List<BleDevice> get discovered => List.unmodifiable(_discovered);

  /// Check if BLE is available and supported.
  Future<bool> get isAvailable async {
    return await FlutterBluePlus.isSupported;
  }

  /// Request BLE permissions.
  Future<bool> requestPermissions() async {
    final btScan = await Permission.bluetoothScan.request();
    final btConnect = await Permission.bluetoothConnect.request();
    final location = await Permission.locationWhenInUse.request();
    return btScan.isGranted && btConnect.isGranted && location.isGranted;
  }

  /// Start scanning for BLE devices.
  Future<void> startScan({Duration timeout = const Duration(seconds: 10)}) async {
    if (!await requestPermissions()) return;

    _discovered.clear();
    _devicesController.add(_discovered);

    await FlutterBluePlus.startScan(timeout: timeout);

    _scanSub = FlutterBluePlus.scanResults.listen((results) {
      _discovered.clear();
      for (final r in results) {
        _discovered.add(BleDevice(
          id: r.device.remoteId.str,
          name: r.device.platformName.isNotEmpty ? r.device.platformName : 'Unknown',
          rssi: r.rssi,
          connectable: r.advertisementData.connectable,
          services: r.advertisementData.serviceUuids.map((e) => e.str).toList(),
          txPower: r.advertisementData.txPowerLevel,
          device: r.device,
        ));
      }
      _devicesController.add(List.from(_discovered));
    });
  }

  /// Stop scanning.
  Future<void> stopScan() async {
    await FlutterBluePlus.stopScan();
    await _scanSub?.cancel();
    _scanSub = null;
  }

  /// Connect to a BLE device.
  Future<BluetoothDevice?> connect(String deviceId) async {
    final match = _discovered.where((d) => d.id == deviceId);
    if (match.isEmpty) return null;

    final device = match.first.device;
    await device.connect(timeout: const Duration(seconds: 10));
    return device;
  }

  /// Disconnect from a BLE device.
  Future<void> disconnect(BluetoothDevice device) async {
    await device.disconnect();
  }

  /// Discover services on a connected device.
  Future<List<BluetoothService>> discoverServices(BluetoothDevice device) async {
    return await device.discoverServices();
  }

  /// Write data to a characteristic.
  Future<void> writeCharacteristic(
    BluetoothCharacteristic characteristic,
    List<int> data,
  ) async {
    await characteristic.write(data);
  }

  /// Read data from a characteristic.
  Future<List<int>> readCharacteristic(BluetoothCharacteristic characteristic) async {
    return await characteristic.read();
  }

  /// Subscribe to characteristic notifications.
  Stream<List<int>> subscribeCharacteristic(BluetoothCharacteristic characteristic) {
    characteristic.setNotifyValue(true);
    return characteristic.lastValueStream;
  }

  void dispose() {
    _scanSub?.cancel();
    _devicesController.close();
  }
}

/// A discovered BLE device.
class BleDevice {
  final String id;
  final String name;
  final int rssi;
  final bool connectable;
  final List<String> services;
  final int? txPower;
  final BluetoothDevice device;

  BleDevice({
    required this.id,
    required this.name,
    required this.rssi,
    required this.connectable,
    required this.services,
    this.txPower,
    required this.device,
  });

  /// Signal strength category.
  String get signalStrength {
    if (rssi > -50) return 'Excellent';
    if (rssi > -70) return 'Good';
    if (rssi > -85) return 'Fair';
    return 'Weak';
  }
}
