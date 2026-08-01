import 'package:flutter/material.dart';

/// Settings page — app configuration, security, and system info.
class SettingsPage extends StatefulWidget {
  const SettingsPage({super.key});

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  bool _darkMode = true;
  bool _e2ee = true;
  bool _autoSync = true;
  bool _meshAutoConnect = true;
  bool _mfa = false;

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
                Text('Settings',
                    style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                          fontWeight: FontWeight.w700, color: Colors.white)),
                const SizedBox(height: 24),
                _section('Security', [
                  _toggle('End-to-End Encryption', 'AES-256-GCM for all data', Icons.lock_rounded,
                      const Color(0xFF00D4AA), _e2ee, (v) => setState(() => _e2ee = v)),
                  _toggle('Multi-Factor Auth', 'TOTP + Recovery Codes', Icons.security_rounded,
                      const Color(0xFF6366F1), _mfa, (v) => setState(() => _mfa = v)),
                ]),
                const SizedBox(height: 20),
                _section('Network', [
                  _toggle('Auto-Connect Mesh', 'Join nearby mesh on startup', Icons.hub_rounded,
                      const Color(0xFF14B8A6), _meshAutoConnect, (v) => setState(() => _meshAutoConnect = v)),
                  _toggle('Auto Sync', 'CRDT sync when peers available', Icons.sync_rounded,
                      const Color(0xFFF59E0B), _autoSync, (v) => setState(() => _autoSync = v)),
                ]),
                const SizedBox(height: 20),
                _section('Appearance', [
                  _toggle('Dark Mode', 'Reduce eye strain', Icons.dark_mode_rounded,
                      const Color(0xFF8B5CF6), _darkMode, (v) => setState(() => _darkMode = v)),
                ]),
                const SizedBox(height: 20),
                _section('System Info', [
                  _infoTile('Version', '0.1.0'),
                  _infoTile('Rust Core', '8 crates • 112 tests'),
                  _infoTile('Node ID', 'a1b2c3d4...'),
                  _infoTile('Platform', Theme.of(context).platform.name),
                ]),
                const SizedBox(height: 24),
                Center(
                  child: Text('WIOS v0.1.0 • Apache-2.0',
                      style: TextStyle(color: Colors.white.withValues(alpha: 0.3), fontSize: 12)),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }

  Widget _section(String title, List<Widget> children) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(title, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        ...children,
      ],
    );
  }

  Widget _toggle(String title, String subtitle, IconData icon, Color color, bool value, ValueChanged<bool> onChanged) {
    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        color: const Color(0xFF141829),
        border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
      ),
      child: Row(
        children: [
          Container(
            width: 40, height: 40,
            decoration: BoxDecoration(borderRadius: BorderRadius.circular(10), color: color.withValues(alpha: 0.15)),
            child: Icon(icon, color: color, size: 20),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(title, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 14)),
                Text(subtitle, style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 12)),
              ],
            ),
          ),
          Switch(
            value: value,
            onChanged: onChanged,
            activeThumbColor: color,
          ),
        ],
      ),
    );
  }

  Widget _infoTile(String label, String value) {
    return Container(
      margin: const EdgeInsets.only(bottom: 8),
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 14),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        color: const Color(0xFF141829),
        border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
      ),
      child: Row(
        children: [
          Text(label, style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
          const Spacer(),
          Text(value, style: const TextStyle(color: Colors.white, fontSize: 13, fontWeight: FontWeight.w500)),
        ],
      ),
    );
  }
}
