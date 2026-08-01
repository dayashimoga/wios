import Flutter
import UIKit
import NearbyInteraction

/// UWB ranging handler for iOS using Nearby Interaction framework.
/// Supported: iPhone 11+ with U1/U2 chip.
@available(iOS 14.0, *)
class UwbHandler: NSObject, FlutterPlugin, NISessionDelegate {

    private var eventSink: FlutterEventSink?
    private var session: NISession?
    private var currentPeerId: String?

    static func register(with registrar: FlutterPluginRegistrar) {
        let channel = FlutterMethodChannel(name: "com.wios/uwb", binaryMessenger: registrar.messenger())
        let instance = UwbHandler()
        registrar.addMethodCallDelegate(instance, channel: channel)

        let eventChannel = FlutterEventChannel(name: "com.wios/uwb/events", binaryMessenger: registrar.messenger())
        eventChannel.setStreamHandler(instance)
    }

    func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
        switch call.method {
        case "isSupported":
            result(NISession.isSupported)
        case "startRanging":
            if let args = call.arguments as? [String: Any],
               let peerId = args["peerId"] as? String,
               let peerToken = args["peerToken"] as? FlutterStandardTypedData {
                startRanging(peerId: peerId, peerToken: peerToken.data, result: result)
            } else {
                result(false)
            }
        case "stopRanging":
            stopRanging(result: result)
        case "getLocalToken":
            getLocalToken(result: result)
        default:
            result(FlutterMethodNotImplemented)
        }
    }

    private func startRanging(peerId: String, peerToken: Data, result: FlutterResult) {
        guard NISession.isSupported else {
            result(false)
            return
        }

        session?.invalidate()
        session = NISession()
        session?.delegate = self
        currentPeerId = peerId

        // Create discovery token from peer's shared token
        if let peerDiscoveryToken = try? NSKeyedUnarchiver.unarchivedObject(ofClass: NIDiscoveryToken.self, from: peerToken) {
            let config = NINearbyPeerConfiguration(peerToken: peerDiscoveryToken)
            session?.run(config)
            result(true)
        } else {
            result(false)
        }
    }

    private func stopRanging(result: FlutterResult) {
        session?.invalidate()
        session = nil
        currentPeerId = nil
        result(true)
    }

    private func getLocalToken(result: FlutterResult) {
        guard NISession.isSupported else {
            result(nil)
            return
        }

        let tempSession = NISession()
        if let token = tempSession.discoveryToken,
           let data = try? NSKeyedArchiver.archivedData(withRootObject: token, requiringSecureCoding: true) {
            result(FlutterStandardTypedData(bytes: data))
        } else {
            result(nil)
        }
        tempSession.invalidate()
    }

    // MARK: - NISessionDelegate

    func session(_ session: NISession, didUpdate nearbyObjects: [NINearbyObject]) {
        for obj in nearbyObjects {
            var resultMap: [String: Any] = [
                "type": "rangeResult",
                "peerId": currentPeerId ?? "unknown",
                "confidence": 1.0
            ]

            if let distance = obj.distance {
                resultMap["distance"] = distance
            }

            if #available(iOS 16.0, *) {
                if let direction = obj.horizontalAngle {
                    resultMap["azimuth"] = direction * 180.0 / .pi // Convert radians to degrees
                }
            }

            DispatchQueue.main.async {
                self.eventSink?(resultMap)
            }
        }
    }

    func session(_ session: NISession, didRemove nearbyObjects: [NINearbyObject], reason: NINearbyObject.RemovalReason) {
        DispatchQueue.main.async {
            self.eventSink?(["type": "peerLost", "peerId": self.currentPeerId ?? "unknown"])
        }
    }

    func session(_ session: NISession, didInvalidateWith error: Error) {
        DispatchQueue.main.async {
            self.eventSink?(["type": "error", "message": error.localizedDescription])
        }
    }

    func sessionWasSuspended(_ session: NISession) {}
    func sessionSuspensionEnded(_ session: NISession) {}
}

// MARK: - FlutterStreamHandler
@available(iOS 14.0, *)
extension UwbHandler: FlutterStreamHandler {
    func onListen(withArguments arguments: Any?, eventSink events: @escaping FlutterEventSink) -> FlutterError? {
        eventSink = events
        return nil
    }
    func onCancel(withArguments arguments: Any?) -> FlutterError? {
        eventSink = nil
        return nil
    }
}
