import 'package:flutter/material.dart';

/// SOS Emergency page — alert management, priority broadcasts, responder tracking.
class SosPage extends StatefulWidget {
  const SosPage({super.key});

  @override
  State<SosPage> createState() => _SosPageState();
}

class _SosPageState extends State<SosPage> with SingleTickerProviderStateMixin {
  late AnimationController _pulseAnim;
  bool _sosTriggered = false;

  final List<_SosAlert> _alerts = [
    _SosAlert('Medical Emergency', 'Building B, Floor 2', 'Critical', '2 min ago', 3, false),
    _SosAlert('Fire Alarm', 'Server Room A', 'LifeThreatening', '5 min ago', 5, true),
    _SosAlert('Security Alert', 'Parking Lot', 'Warning', '12 min ago', 1, false),
  ];

  @override
  void initState() {
    super.initState();
    _pulseAnim = AnimationController(vsync: this, duration: const Duration(milliseconds: 800))..repeat(reverse: true);
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
          _buildSosButton(),
          const Padding(
            padding: EdgeInsets.fromLTRB(16, 16, 16, 8),
            child: Align(
              alignment: Alignment.centerLeft,
              child: Text('Active Alerts', style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.w600)),
            ),
          ),
          Expanded(child: _buildAlertList()),
        ],
      ),
    );
  }

  Widget _buildHeader() {
    final active = _alerts.where((a) => !a.resolved).length;
    return Container(
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(10),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12),
              color: const Color(0xFFEF4444).withValues(alpha: 0.15),
            ),
            child: const Icon(Icons.sos_rounded, color: Color(0xFFEF4444), size: 24),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Emergency SOS', style: TextStyle(color: Colors.white, fontSize: 20, fontWeight: FontWeight.w700)),
                Text('$active active alerts', style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildSosButton() {
    return AnimatedBuilder(
      animation: _pulseAnim,
      builder: (context, _) {
        final scale = _sosTriggered ? 1.0 + _pulseAnim.value * 0.05 : 1.0;
        return Center(
          child: GestureDetector(
            onLongPress: () => setState(() => _sosTriggered = !_sosTriggered),
            child: Transform.scale(
              scale: scale,
              child: Container(
                width: 120, height: 120,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  gradient: LinearGradient(
                    begin: Alignment.topLeft, end: Alignment.bottomRight,
                    colors: _sosTriggered
                        ? [const Color(0xFFEF4444), const Color(0xFFB91C1C)]
                        : [const Color(0xFF1F2937), const Color(0xFF111827)],
                  ),
                  boxShadow: _sosTriggered ? [
                    BoxShadow(color: const Color(0xFFEF4444).withValues(alpha: 0.4), blurRadius: 30, spreadRadius: 5),
                  ] : [],
                ),
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(Icons.sos_rounded, color: _sosTriggered ? Colors.white : Colors.white.withValues(alpha: 0.4), size: 32),
                    const SizedBox(height: 4),
                    Text(_sosTriggered ? 'ACTIVE' : 'HOLD',
                        style: TextStyle(
                          color: _sosTriggered ? Colors.white : Colors.white.withValues(alpha: 0.4),
                          fontSize: 11, fontWeight: FontWeight.w700, letterSpacing: 1.5,
                        )),
                  ],
                ),
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildAlertList() {
    return ListView.builder(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      itemCount: _alerts.length,
      itemBuilder: (context, i) {
        final a = _alerts[i];
        final Color severityColor;
        switch (a.severity) {
          case 'LifeThreatening': severityColor = const Color(0xFFEF4444); break;
          case 'Critical': severityColor = const Color(0xFFF59E0B); break;
          case 'Warning': severityColor = const Color(0xFF3B82F6); break;
          default: severityColor = Colors.white.withValues(alpha: 0.3);
        }

        return Container(
          margin: const EdgeInsets.only(bottom: 10),
          padding: const EdgeInsets.all(14),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(12),
            color: const Color(0xFF141829),
            border: Border.all(color: a.resolved ? Colors.white.withValues(alpha: 0.04) : severityColor.withValues(alpha: 0.2)),
          ),
          child: Row(
            children: [
              Container(
                width: 4, height: 44,
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(2),
                  color: a.resolved ? Colors.white.withValues(alpha: 0.1) : severityColor,
                ),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Text(a.title, style: TextStyle(
                          color: a.resolved ? Colors.white.withValues(alpha: 0.4) : Colors.white,
                          fontSize: 14, fontWeight: FontWeight.w600,
                          decoration: a.resolved ? TextDecoration.lineThrough : null,
                        )),
                        const Spacer(),
                        Container(
                          padding: const EdgeInsets.symmetric(horizontal: 6, vertical: 2),
                          decoration: BoxDecoration(
                            borderRadius: BorderRadius.circular(4),
                            color: severityColor.withValues(alpha: 0.15),
                          ),
                          child: Text(a.severity, style: TextStyle(color: severityColor, fontSize: 10, fontWeight: FontWeight.w600)),
                        ),
                      ],
                    ),
                    const SizedBox(height: 4),
                    Text('${a.location} • ${a.time}', style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 12)),
                    Text('${a.responders} responders', style: TextStyle(color: Colors.white.withValues(alpha: 0.3), fontSize: 11)),
                  ],
                ),
              ),
              if (!a.resolved)
                IconButton(
                  icon: const Icon(Icons.check_circle_outline_rounded, color: Color(0xFF00D4AA), size: 22),
                  onPressed: () => setState(() => a.resolved = true),
                ),
            ],
          ),
        );
      },
    );
  }
}

class _SosAlert {
  final String title;
  final String location;
  final String severity;
  final String time;
  final int responders;
  bool resolved;

  _SosAlert(this.title, this.location, this.severity, this.time, this.responders, this.resolved);
}
