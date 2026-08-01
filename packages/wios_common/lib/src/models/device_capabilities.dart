/// Device capabilities model.
class DeviceCapabilities {
  final int cpuCores;
  final int ramMb;
  final int storageMb;
  final bool hasGpu;
  final bool hasCamera;
  final bool hasMicrophone;
  final bool hasBluetooth;
  final bool hasWifiDirect;
  final bool hasNfc;
  final double? batteryLevel;
  final List<String> aiEngines;

  const DeviceCapabilities({
    required this.cpuCores,
    required this.ramMb,
    required this.storageMb,
    this.hasGpu = false,
    this.hasCamera = false,
    this.hasMicrophone = false,
    this.hasBluetooth = false,
    this.hasWifiDirect = false,
    this.hasNfc = false,
    this.batteryLevel,
    this.aiEngines = const [],
  });

  factory DeviceCapabilities.fromJson(Map<String, dynamic> json) =>
      DeviceCapabilities(
        cpuCores: json['cpu_cores'] as int,
        ramMb: json['ram_mb'] as int,
        storageMb: json['storage_mb'] as int,
        hasGpu: json['has_gpu'] as bool? ?? false,
        hasCamera: json['has_camera'] as bool? ?? false,
        hasMicrophone: json['has_microphone'] as bool? ?? false,
        hasBluetooth: json['has_bluetooth'] as bool? ?? false,
        hasWifiDirect: json['has_wifi_direct'] as bool? ?? false,
        hasNfc: json['has_nfc'] as bool? ?? false,
        batteryLevel: (json['battery_level'] as num?)?.toDouble(),
        aiEngines: (json['ai_engines'] as List<dynamic>?)
                ?.cast<String>() ??
            const [],
      );

  Map<String, dynamic> toJson() => {
        'cpu_cores': cpuCores,
        'ram_mb': ramMb,
        'storage_mb': storageMb,
        'has_gpu': hasGpu,
        'has_camera': hasCamera,
        'has_microphone': hasMicrophone,
        'has_bluetooth': hasBluetooth,
        'has_wifi_direct': hasWifiDirect,
        'has_nfc': hasNfc,
        'battery_level': batteryLevel,
        'ai_engines': aiEngines,
      };
}
