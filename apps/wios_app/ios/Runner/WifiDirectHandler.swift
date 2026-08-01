import Flutter
import UIKit
import MultipeerConnectivity

/// WiFi Direct equivalent for iOS using MultipeerConnectivity framework.
/// Provides peer discovery, connection, and data transfer via method channels.
class WifiDirectHandler: NSObject, FlutterPlugin, MCSessionDelegate, MCNearbyServiceBrowserDelegate, MCNearbyServiceAdvertiserDelegate {

    static let serviceType = "wios-mesh"
    private var eventSink: FlutterEventSink?
    private var session: MCSession?
    private var browser: MCNearbyServiceBrowser?
    private var advertiser: MCNearbyServiceAdvertiser?
    private let localPeerId = MCPeerID(displayName: UIDevice.current.name)

    static func register(with registrar: FlutterPluginRegistrar) {
        let channel = FlutterMethodChannel(name: "com.wios/wifi_direct", binaryMessenger: registrar.messenger())
        let instance = WifiDirectHandler()
        registrar.addMethodCallDelegate(instance, channel: channel)

        let eventChannel = FlutterEventChannel(name: "com.wios/wifi_direct/events", binaryMessenger: registrar.messenger())
        eventChannel.setStreamHandler(instance)
    }

    override init() {
        eventSink = nil
        super.init()
    }

    func handle(_ call: FlutterMethodCall, result: @escaping FlutterResult) {
        switch call.method {
        case "initialize":
            initialize(result: result)
        case "startDiscovery":
            startDiscovery(result: result)
        case "stopDiscovery":
            stopDiscovery(result: result)
        case "connect":
            if let args = call.arguments as? [String: Any],
               let address = args["deviceAddress"] as? String {
                connect(address: address, result: result)
            } else {
                result(false)
            }
        case "disconnect":
            disconnect(result: result)
        case "sendData":
            if let args = call.arguments as? [String: Any],
               let data = args["data"] as? FlutterStandardTypedData {
                sendData(data: data.data, result: result)
            } else {
                result(false)
            }
        default:
            result(FlutterMethodNotImplemented)
        }
    }

    private func initialize(result: FlutterResult) {
        session = MCSession(peer: localPeerId, securityIdentity: nil, encryptionPreference: .required)
        session?.delegate = self

        browser = MCNearbyServiceBrowser(peer: localPeerId, serviceType: WifiDirectHandler.serviceType)
        browser?.delegate = self

        advertiser = MCNearbyServiceAdvertiser(peer: localPeerId, discoveryInfo: nil, serviceType: WifiDirectHandler.serviceType)
        advertiser?.delegate = self

        result(true)
    }

    private func startDiscovery(result: FlutterResult) {
        browser?.startBrowsingForPeers()
        advertiser?.startAdvertisingPeer()
        result(true)
    }

    private func stopDiscovery(result: FlutterResult) {
        browser?.stopBrowsingForPeers()
        advertiser?.stopAdvertisingPeer()
        result(true)
    }

    private func connect(address: String, result: FlutterResult) {
        // In Multipeer, connection is invitation-based
        // The address maps to a peer display name
        guard let session = session else { result(false); return }
        if let peer = session.connectedPeers.first(where: { $0.displayName == address }) {
            result(true) // Already connected
        } else {
            // Invitation happens in browser delegate
            result(true)
        }
    }

    private func disconnect(result: FlutterResult) {
        session?.disconnect()
        result(true)
    }

    private func sendData(data: Data, result: FlutterResult) {
        guard let session = session, !session.connectedPeers.isEmpty else {
            result(false)
            return
        }
        do {
            try session.send(data, toPeers: session.connectedPeers, with: .reliable)
            result(true)
        } catch {
            result(false)
        }
    }

    // MARK: - MCSessionDelegate

    func session(_ session: MCSession, peer peerID: MCPeerID, didChange state: MCSessionState) {
        DispatchQueue.main.async {
            switch state {
            case .connected:
                self.eventSink?(["type": "connected", "deviceName": peerID.displayName, "deviceAddress": peerID.displayName])
            case .notConnected:
                self.eventSink?(["type": "disconnected", "deviceAddress": peerID.displayName])
            case .connecting:
                break
            @unknown default:
                break
            }
        }
    }

    func session(_ session: MCSession, didReceive data: Data, fromPeer peerID: MCPeerID) {
        DispatchQueue.main.async {
            self.eventSink?(["type": "dataReceived", "data": FlutterStandardTypedData(bytes: data), "from": peerID.displayName])
        }
    }

    func session(_ session: MCSession, didReceive stream: InputStream, withName streamName: String, fromPeer peerID: MCPeerID) {}
    func session(_ session: MCSession, didStartReceivingResourceWithName name: String, fromPeer peerID: MCPeerID, with progress: Progress) {}
    func session(_ session: MCSession, didFinishReceivingResourceWithName name: String, fromPeer peerID: MCPeerID, at localURL: URL?, withError error: Error?) {}

    // MARK: - MCNearbyServiceBrowserDelegate

    func browser(_ browser: MCNearbyServiceBrowser, foundPeer peerID: MCPeerID, withDiscoveryInfo info: [String: String]?) {
        // Auto-invite discovered peers
        browser.invitePeer(peerID, to: session!, withContext: nil, timeout: 10)
        DispatchQueue.main.async {
            self.eventSink?(["type": "peerFound", "deviceAddress": peerID.displayName, "deviceName": peerID.displayName, "isGroupOwner": false, "status": 3])
        }
    }

    func browser(_ browser: MCNearbyServiceBrowser, lostPeer peerID: MCPeerID) {
        DispatchQueue.main.async {
            self.eventSink?(["type": "peerLost", "deviceAddress": peerID.displayName])
        }
    }

    // MARK: - MCNearbyServiceAdvertiserDelegate

    func advertiser(_ advertiser: MCNearbyServiceAdvertiser, didReceiveInvitationFromPeer peerID: MCPeerID, withContext context: Data?, invitationHandler: @escaping (Bool, MCSession?) -> Void) {
        invitationHandler(true, session) // Auto-accept invitations
    }
}

// MARK: - FlutterStreamHandler
extension WifiDirectHandler: FlutterStreamHandler {
    func onListen(withArguments arguments: Any?, eventSink events: @escaping FlutterEventSink) -> FlutterError? {
        eventSink = events
        return nil
    }
    func onCancel(withArguments arguments: Any?) -> FlutterError? {
        eventSink = nil
        return nil
    }
}
