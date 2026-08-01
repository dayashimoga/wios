import 'package:flutter/material.dart';
import 'dart:math' as math;

/// Mesh Network page — shows peer topology, connections, and discovery.
class MeshPage extends StatefulWidget {
  const MeshPage({super.key});

  @override
  State<MeshPage> createState() => _MeshPageState();
}

class _MeshPageState extends State<MeshPage> with SingleTickerProviderStateMixin {
  late AnimationController _animController;
  bool _isScanning = false;

  final List<_PeerInfo> _demoPeers = [
    _PeerInfo('Local Node', '192.168.1.10', 'Online', 1.0, true),
    _PeerInfo('Kitchen Hub', '192.168.1.15', 'Online', 0.85, false),
    _PeerInfo('Office PC', '192.168.1.22', 'Online', 0.72, false),
    _PeerInfo('Garage Sensor', '192.168.1.30', 'Weak', 0.35, false),
  ];

  @override
  void initState() {
    super.initState();
    _animController = AnimationController(
      vsync: this,
      duration: const Duration(seconds: 6),
    )..repeat();
  }

  @override
  void dispose() {
    _animController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return CustomScrollView(
      slivers: [
        SliverToBoxAdapter(
          child: Padding(
            padding: const EdgeInsets.all(24),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                _buildHeader(context),
                const SizedBox(height: 24),
                _buildTopologyCard(),
                const SizedBox(height: 20),
                _buildStatsRow(),
                const SizedBox(height: 20),
                _buildPeerList(),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildHeader(BuildContext context) {
    return Row(
      children: [
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Mesh Network',
                style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                      fontWeight: FontWeight.w700,
                      color: Colors.white,
                    )),
            const SizedBox(height: 4),
            Text('${_demoPeers.length} peers on local mesh',
                style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 14)),
          ],
        ),
        const Spacer(),
        FilledButton.icon(
          onPressed: () => setState(() => _isScanning = !_isScanning),
          icon: Icon(_isScanning ? Icons.stop_rounded : Icons.radar_rounded, size: 18),
          label: Text(_isScanning ? 'Stop' : 'Scan'),
          style: FilledButton.styleFrom(
            backgroundColor: const Color(0xFF00D4AA),
            foregroundColor: Colors.black,
          ),
        ),
      ],
    );
  }

  Widget _buildTopologyCard() {
    return Container(
      height: 260,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        color: const Color(0xFF141829),
        border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
      ),
      child: AnimatedBuilder(
        animation: _animController,
        builder: (context, _) {
          return CustomPaint(
            size: const Size(double.infinity, 260),
            painter: _TopologyPainter(_animController.value, _demoPeers.length),
          );
        },
      ),
    );
  }

  Widget _buildStatsRow() {
    return Row(
      children: [
        _buildMiniStat('Peers', '${_demoPeers.length}', const Color(0xFF00D4AA)),
        const SizedBox(width: 12),
        _buildMiniStat('Latency', '12ms', const Color(0xFF6366F1)),
        const SizedBox(width: 12),
        _buildMiniStat('Bandwidth', '54 Mbps', const Color(0xFFF59E0B)),
        const SizedBox(width: 12),
        _buildMiniStat('Msgs/s', '128', const Color(0xFFEC4899)),
      ],
    );
  }

  Widget _buildMiniStat(String label, String value, Color color) {
    return Expanded(
      child: Container(
        padding: const EdgeInsets.all(16),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(12),
          color: const Color(0xFF141829),
          border: Border.all(color: color.withValues(alpha: 0.2)),
        ),
        child: Column(
          children: [
            Text(value,
                style: TextStyle(color: color, fontSize: 20, fontWeight: FontWeight.w700)),
            const SizedBox(height: 4),
            Text(label,
                style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 11)),
          ],
        ),
      ),
    );
  }

  Widget _buildPeerList() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Connected Peers',
            style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        ..._demoPeers.map(_buildPeerTile),
      ],
    );
  }

  Widget _buildPeerTile(_PeerInfo peer) {
    final color = peer.signal > 0.6
        ? const Color(0xFF00D4AA)
        : peer.signal > 0.3
            ? const Color(0xFFF59E0B)
            : const Color(0xFFEF4444);
    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        color: const Color(0xFF141829),
        border: Border.all(
          color: peer.isLocal ? const Color(0xFF00D4AA).withValues(alpha: 0.3) : Colors.white.withValues(alpha: 0.06),
        ),
      ),
      child: Row(
        children: [
          Container(
            width: 40,
            height: 40,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(10),
              color: color.withValues(alpha: 0.15),
            ),
            child: Icon(
              peer.isLocal ? Icons.phone_android_rounded : Icons.devices_rounded,
              color: color,
              size: 20,
            ),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(peer.name,
                    style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 14)),
                Text(peer.address,
                    style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 12)),
              ],
            ),
          ),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(6),
              color: color.withValues(alpha: 0.1),
            ),
            child: Text(peer.status,
                style: TextStyle(color: color, fontSize: 11, fontWeight: FontWeight.w600)),
          ),
          const SizedBox(width: 8),
          SizedBox(
            width: 40,
            child: _buildSignalBars(peer.signal, color),
          ),
        ],
      ),
    );
  }

  Widget _buildSignalBars(double strength, Color color) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.end,
      children: List.generate(4, (i) {
        final isActive = strength > (i / 4);
        return Container(
          width: 4,
          height: 6.0 + i * 4,
          margin: const EdgeInsets.symmetric(horizontal: 1),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(2),
            color: isActive ? color : Colors.white.withValues(alpha: 0.1),
          ),
        );
      }),
    );
  }
}

class _PeerInfo {
  final String name, address, status;
  final double signal;
  final bool isLocal;
  _PeerInfo(this.name, this.address, this.status, this.signal, this.isLocal);
}

class _TopologyPainter extends CustomPainter {
  final double t;
  final int peerCount;
  _TopologyPainter(this.t, this.peerCount);

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, size.height / 2);
    final nodes = <Offset>[center];
    for (int i = 0; i < peerCount - 1; i++) {
      final angle = (i / (peerCount - 1)) * 2 * math.pi + t * 2 * math.pi * 0.3;
      final r = size.width * 0.18 + math.sin(angle * 2) * 20;
      nodes.add(Offset(center.dx + math.cos(angle) * r, center.dy + math.sin(angle) * r * 0.7));
    }

    final linePaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1.0;
    for (int i = 1; i < nodes.length; i++) {
      linePaint.color = const Color(0xFF00D4AA).withValues(alpha: 0.2);
      canvas.drawLine(nodes[0], nodes[i], linePaint);
      for (int j = i + 1; j < nodes.length; j++) {
        if ((nodes[i] - nodes[j]).distance < size.width * 0.3) {
          linePaint.color = const Color(0xFF6366F1).withValues(alpha: 0.12);
          canvas.drawLine(nodes[i], nodes[j], linePaint);
        }
      }
    }

    final nodePaint = Paint()..style = PaintingStyle.fill;
    for (int i = 0; i < nodes.length; i++) {
      nodePaint.color = i == 0 ? const Color(0xFF00D4AA) : const Color(0xFF6366F1).withValues(alpha: 0.7);
      canvas.drawCircle(nodes[i], i == 0 ? 6 : 4, nodePaint);
      if (i == 0) {
        nodePaint.color = const Color(0xFF00D4AA).withValues(alpha: 0.15);
        canvas.drawCircle(nodes[i], 14 + 4 * math.sin(t * 2 * math.pi), nodePaint);
      }
    }
  }

  @override
  bool shouldRepaint(covariant _TopologyPainter old) => old.t != t;
}
