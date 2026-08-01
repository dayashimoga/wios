import 'package:flutter/material.dart';
import 'dart:math' as math;

/// Sensing page — RSSI positioning, RF heatmap, and sensor data.
class SensingPage extends StatefulWidget {
  const SensingPage({super.key});

  @override
  State<SensingPage> createState() => _SensingPageState();
}

class _SensingPageState extends State<SensingPage> with SingleTickerProviderStateMixin {
  late AnimationController _heatmapAnim;

  @override
  void initState() {
    super.initState();
    _heatmapAnim = AnimationController(vsync: this, duration: const Duration(seconds: 4))..repeat();
  }

  @override
  void dispose() {
    _heatmapAnim.dispose();
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
                _buildHeatmapCard(),
                const SizedBox(height: 20),
                _buildBeaconList(),
                const SizedBox(height: 20),
                _buildPositionInfo(),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _buildHeader(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text('Wireless Sensing',
            style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                  fontWeight: FontWeight.w700, color: Colors.white)),
        const SizedBox(height: 4),
        Text('RSSI positioning • RF heatmap • Signal analytics',
            style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 14)),
      ],
    );
  }

  Widget _buildHeatmapCard() {
    return Container(
      height: 280,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        color: const Color(0xFF141829),
        border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
      ),
      child: Stack(
        children: [
          AnimatedBuilder(
            animation: _heatmapAnim,
            builder: (context, _) {
              return CustomPaint(
                size: const Size(double.infinity, 280),
                painter: _HeatmapPainter(_heatmapAnim.value),
              );
            },
          ),
          Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('RF Heatmap',
                    style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
                const Spacer(),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
                  decoration: BoxDecoration(
                    borderRadius: BorderRadius.circular(8),
                    color: const Color(0xFF14B8A6).withValues(alpha: 0.15),
                  ),
                  child: const Text('Scanning',
                      style: TextStyle(color: Color(0xFF14B8A6), fontSize: 12, fontWeight: FontWeight.w600)),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildBeaconList() {
    final beacons = [
      ('AP-Living-Room', -42, '5 GHz', 2.1),
      ('AP-Kitchen', -58, '2.4 GHz', 5.3),
      ('BLE-Sensor-01', -67, 'BLE', 8.7),
      ('AP-Garage', -78, '2.4 GHz', 14.2),
    ];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Detected Beacons',
            style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        ...beacons.map((b) {
          final color = b.$2 > -50
              ? const Color(0xFF00D4AA)
              : b.$2 > -65
                  ? const Color(0xFFF59E0B)
                  : const Color(0xFFEF4444);
          return Container(
            margin: const EdgeInsets.only(bottom: 8),
            padding: const EdgeInsets.all(14),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12),
              color: const Color(0xFF141829),
              border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
            ),
            child: Row(
              children: [
                Icon(b.$3 == 'BLE' ? Icons.bluetooth_rounded : Icons.wifi_rounded,
                    color: color, size: 20),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(b.$1, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 13)),
                      Text('${b.$3} • ${b.$4.toStringAsFixed(1)}m away',
                          style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 11)),
                    ],
                  ),
                ),
                Text('${b.$2} dBm', style: TextStyle(color: color, fontWeight: FontWeight.w700, fontSize: 13)),
              ],
            ),
          );
        }),
      ],
    );
  }

  Widget _buildPositionInfo() {
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        gradient: LinearGradient(
          colors: [const Color(0xFF14B8A6).withValues(alpha: 0.1), const Color(0xFF141829)],
        ),
        border: Border.all(color: const Color(0xFF14B8A6).withValues(alpha: 0.2)),
      ),
      child: Row(
        children: [
          Container(
            width: 48, height: 48,
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12),
              color: const Color(0xFF14B8A6).withValues(alpha: 0.2),
            ),
            child: const Icon(Icons.my_location_rounded, color: Color(0xFF14B8A6), size: 24),
          ),
          const SizedBox(width: 16),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Estimated Position',
                    style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 15)),
                const SizedBox(height: 4),
                Text('(4.2, 6.8) ± 1.5m • Living Room',
                    style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _HeatmapPainter extends CustomPainter {
  final double t;
  _HeatmapPainter(this.t);

  @override
  void paint(Canvas canvas, Size size) {
    // Simulated heatmap gradient spots
    final spots = [
      (0.3, 0.4, 0.8),
      (0.7, 0.6, 0.5),
      (0.5, 0.3, 0.3),
    ];

    for (final spot in spots) {
      final center = Offset(
        size.width * spot.$1 + math.sin(t * 2 * math.pi) * 10,
        size.height * spot.$2 + math.cos(t * 2 * math.pi) * 8,
      );
      final radius = size.width * 0.2 * spot.$3;
      final gradient = RadialGradient(
        colors: [
          const Color(0xFF00D4AA).withValues(alpha: 0.3 * spot.$3),
          const Color(0xFF00D4AA).withValues(alpha: 0.05),
          Colors.transparent,
        ],
      );
      final paint = Paint()..shader = gradient.createShader(Rect.fromCircle(center: center, radius: radius));
      canvas.drawCircle(center, radius, paint);
    }
  }

  @override
  bool shouldRepaint(covariant _HeatmapPainter old) => old.t != t;
}
