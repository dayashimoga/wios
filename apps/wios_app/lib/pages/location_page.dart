import 'package:flutter/material.dart';

/// Location/navigation page — floor plans, indoor positioning, asset tracking.
class LocationPage extends StatefulWidget {
  const LocationPage({super.key});

  @override
  State<LocationPage> createState() => _LocationPageState();
}

class _LocationPageState extends State<LocationPage> with SingleTickerProviderStateMixin {
  late AnimationController _pulseAnim;
  int _selectedFloor = 0;

  final List<_TrackedAsset> _assets = [
    _TrackedAsset('Dev Laptop', Icons.laptop_rounded, 0.3, 0.4, 'Office A', -45),
    _TrackedAsset('Phone', Icons.phone_android_rounded, 0.6, 0.3, 'Lobby', -60),
    _TrackedAsset('Headset', Icons.headset_rounded, 0.5, 0.7, 'Meeting Room', -72),
    _TrackedAsset('Badge', Icons.badge_rounded, 0.2, 0.8, 'Hallway', -55),
  ];

  @override
  void initState() {
    super.initState();
    _pulseAnim = AnimationController(vsync: this, duration: const Duration(seconds: 2))..repeat();
  }

  @override
  void dispose() {
    _pulseAnim.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      color: const Color(0xFF0A0E1A),
      child: Column(
        children: [
          _buildHeader(),
          _buildFloorSelector(),
          Expanded(child: _buildMap()),
          _buildAssetList(),
        ],
      ),
    );
  }

  Widget _buildHeader() {
    return Container(
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(10),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12),
              color: const Color(0xFF3B82F6).withValues(alpha: 0.15),
            ),
            child: const Icon(Icons.location_on_rounded, color: Color(0xFF3B82F6), size: 24),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Indoor Positioning', style: TextStyle(color: Colors.white, fontSize: 20, fontWeight: FontWeight.w700)),
                Text('${_assets.length} assets tracked • Floor $_selectedFloor',
                    style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
              ],
            ),
          ),
          FilledButton.icon(
            onPressed: () {},
            icon: const Icon(Icons.navigation_rounded, size: 18),
            label: const Text('Navigate'),
            style: FilledButton.styleFrom(
              backgroundColor: const Color(0xFF3B82F6),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildFloorSelector() {
    return Container(
      height: 36,
      margin: const EdgeInsets.symmetric(horizontal: 16),
      child: Row(
        children: List.generate(3, (i) {
          final selected = _selectedFloor == i;
          return Padding(
            padding: const EdgeInsets.only(right: 8),
            child: ChoiceChip(
              label: Text('Floor $i', style: TextStyle(
                color: selected ? Colors.black : Colors.white.withValues(alpha: 0.6), fontSize: 12,
              )),
              selected: selected,
              onSelected: (_) => setState(() => _selectedFloor = i),
              selectedColor: const Color(0xFF3B82F6),
              backgroundColor: const Color(0xFF141829),
              side: BorderSide(color: selected ? Colors.transparent : Colors.white.withValues(alpha: 0.08)),
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
            ),
          );
        }),
      ),
    );
  }

  Widget _buildMap() {
    return Container(
      margin: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(14),
        color: const Color(0xFF141829),
        border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
      ),
      child: AnimatedBuilder(
        animation: _pulseAnim,
        builder: (context, _) => CustomPaint(
          size: Size.infinite,
          painter: _FloorMapPainter(_assets, _pulseAnim.value),
        ),
      ),
    );
  }

  Widget _buildAssetList() {
    return Container(
      height: 160,
      padding: const EdgeInsets.only(bottom: 8),
      child: ListView.builder(
        scrollDirection: Axis.horizontal,
        padding: const EdgeInsets.symmetric(horizontal: 16),
        itemCount: _assets.length,
        itemBuilder: (context, i) {
          final a = _assets[i];
          return Container(
            width: 140,
            margin: const EdgeInsets.only(right: 10),
            padding: const EdgeInsets.all(12),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12),
              color: const Color(0xFF141829),
              border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Icon(a.icon, color: const Color(0xFF3B82F6), size: 24),
                const SizedBox(height: 8),
                Text(a.name, style: const TextStyle(color: Colors.white, fontSize: 13, fontWeight: FontWeight.w600)),
                Text(a.zone, style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 11)),
                const Spacer(),
                Row(
                  children: [
                    Icon(Icons.signal_cellular_alt_rounded, color: a.rssi > -60 ? const Color(0xFF00D4AA) : Colors.orange, size: 14),
                    const SizedBox(width: 4),
                    Text('${a.rssi} dBm', style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 11)),
                  ],
                ),
              ],
            ),
          );
        },
      ),
    );
  }
}

class _TrackedAsset {
  final String name;
  final IconData icon;
  final double relX, relY;
  final String zone;
  final int rssi;

  _TrackedAsset(this.name, this.icon, this.relX, this.relY, this.zone, this.rssi);
}

class _FloorMapPainter extends CustomPainter {
  final List<_TrackedAsset> assets;
  final double t;
  _FloorMapPainter(this.assets, this.t);

  @override
  void paint(Canvas canvas, Size size) {
    // Draw grid
    final gridPaint = Paint()..color = Colors.white.withValues(alpha: 0.03)..strokeWidth = 1;
    for (double x = 0; x < size.width; x += 30) {
      canvas.drawLine(Offset(x, 0), Offset(x, size.height), gridPaint);
    }
    for (double y = 0; y < size.height; y += 30) {
      canvas.drawLine(Offset(0, y), Offset(size.width, y), gridPaint);
    }

    // Draw rooms
    final roomPaint = Paint()
      ..color = const Color(0xFF3B82F6).withValues(alpha: 0.08)
      ..style = PaintingStyle.fill;
    canvas.drawRRect(RRect.fromRectAndRadius(Rect.fromLTWH(20, 20, size.width * 0.4, size.height * 0.4), const Radius.circular(6)), roomPaint);
    canvas.drawRRect(RRect.fromRectAndRadius(Rect.fromLTWH(size.width * 0.5, 20, size.width * 0.45, size.height * 0.35), const Radius.circular(6)), roomPaint);
    canvas.drawRRect(RRect.fromRectAndRadius(Rect.fromLTWH(20, size.height * 0.55, size.width * 0.5, size.height * 0.4), const Radius.circular(6)), roomPaint);

    // Draw assets with pulsing circles
    for (final asset in assets) {
      final cx = asset.relX * size.width;
      final cy = asset.relY * size.height;

      // Pulse ring
      final pulseRadius = 12 + t * 15;
      final pulsePaint = Paint()
        ..color = const Color(0xFF3B82F6).withValues(alpha: 0.3 * (1 - t))
        ..style = PaintingStyle.stroke
        ..strokeWidth = 2;
      canvas.drawCircle(Offset(cx, cy), pulseRadius, pulsePaint);

      // Solid dot
      canvas.drawCircle(Offset(cx, cy), 6, Paint()..color = const Color(0xFF3B82F6));
      canvas.drawCircle(Offset(cx, cy), 3, Paint()..color = Colors.white);
    }
  }

  @override
  bool shouldRepaint(covariant _FloorMapPainter old) => true;
}
