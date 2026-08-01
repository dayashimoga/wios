import 'dart:async';
import 'package:camera/camera.dart';
import 'package:permission_handler/permission_handler.dart';

/// Camera & microphone service — capture, preview, and share across mesh.
/// Wires into the Rust DeviceBus (DeviceResource::Camera / Microphone).
class MediaCaptureService {
  CameraController? _cameraController;
  List<CameraDescription> _cameras = [];
  bool _initialized = false;

  bool get isInitialized => _initialized;
  CameraController? get controller => _cameraController;
  List<CameraDescription> get cameras => _cameras;

  /// Request camera and microphone permissions.
  Future<bool> requestPermissions() async {
    final camera = await Permission.camera.request();
    final mic = await Permission.microphone.request();
    return camera.isGranted && mic.isGranted;
  }

  /// Initialize available cameras.
  Future<void> initialize() async {
    if (!await requestPermissions()) return;
    _cameras = await availableCameras();
    if (_cameras.isNotEmpty) {
      await _selectCamera(0);
    }
    _initialized = true;
  }

  /// Select and activate a camera by index.
  Future<void> _selectCamera(int index) async {
    if (index >= _cameras.length) return;

    await _cameraController?.dispose();
    _cameraController = CameraController(
      _cameras[index],
      ResolutionPreset.high,
      enableAudio: true,
      imageFormatGroup: ImageFormatGroup.jpeg,
    );

    await _cameraController!.initialize();
  }

  /// Switch to front/back camera.
  Future<void> switchCamera() async {
    if (_cameras.length < 2) return;
    final current = _cameraController?.description;
    final nextIndex = _cameras.indexOf(current!) == 0 ? 1 : 0;
    await _selectCamera(nextIndex);
  }

  /// Take a photo.
  Future<XFile?> takePhoto() async {
    if (_cameraController == null || !_cameraController!.value.isInitialized) return null;
    return await _cameraController!.takePicture();
  }

  /// Start video recording.
  Future<void> startVideoRecording() async {
    if (_cameraController == null || !_cameraController!.value.isInitialized) return;
    if (_cameraController!.value.isRecordingVideo) return;
    await _cameraController!.startVideoRecording();
  }

  /// Stop video recording and return the file.
  Future<XFile?> stopVideoRecording() async {
    if (_cameraController == null || !_cameraController!.value.isRecordingVideo) return null;
    return await _cameraController!.stopVideoRecording();
  }

  /// Set flash mode.
  Future<void> setFlashMode(FlashMode mode) async {
    await _cameraController?.setFlashMode(mode);
  }

  /// Set zoom level (1.0 = no zoom).
  Future<void> setZoom(double zoom) async {
    final max = await _cameraController?.getMaxZoomLevel() ?? 1.0;
    final min = await _cameraController?.getMinZoomLevel() ?? 1.0;
    await _cameraController?.setZoomLevel(zoom.clamp(min, max));
  }

  /// Get camera info for sharing via DeviceBus.
  Map<String, dynamic> getCameraInfo() {
    if (_cameraController == null) return {};
    final desc = _cameraController!.description;
    return {
      'name': desc.name,
      'direction': desc.lensDirection == CameraLensDirection.front ? 'front' : 'back',
      'sensorOrientation': desc.sensorOrientation,
      'resolution': _cameraController!.value.previewSize?.toString() ?? 'unknown',
      'isRecording': _cameraController!.value.isRecordingVideo,
    };
  }

  void dispose() {
    _cameraController?.dispose();
    _cameraController = null;
    _initialized = false;
  }
}
