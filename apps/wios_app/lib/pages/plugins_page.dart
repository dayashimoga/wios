import 'package:flutter/material.dart';

/// Plugin marketplace page — browse, install, manage plugins.
class PluginsPage extends StatefulWidget {
  const PluginsPage({super.key});

  @override
  State<PluginsPage> createState() => _PluginsPageState();
}

class _PluginsPageState extends State<PluginsPage> {
  int _selectedTab = 0;

  final List<_Plugin> _installed = [
    _Plugin('Smart Lights', 'Automate lighting based on presence', Icons.lightbulb_rounded, '1.2.0', true, 4.5),
    _Plugin('Data Logger', 'Log sensor data to CSV/SQLite', Icons.analytics_rounded, '2.0.1', true, 4.8),
    _Plugin('Mesh Monitor', 'Real-time mesh health dashboard', Icons.monitor_heart_rounded, '1.0.0', false, 4.2),
  ];

  final List<_Plugin> _available = [
    _Plugin('Voice Assistant', 'Local llama.cpp NLP assistant', Icons.record_voice_over_rounded, '0.9.0', false, 4.1),
    _Plugin('File Sync Pro', 'Advanced delta sync across nodes', Icons.sync_rounded, '1.3.0', false, 4.6),
    _Plugin('RF Analyzer', 'Advanced RF spectrum visualization', Icons.radio_rounded, '1.1.0', false, 4.3),
    _Plugin('Geo Fence', 'Advanced geofence automation', Icons.fence_rounded, '0.8.0', false, 3.9),
  ];

  @override
  Widget build(BuildContext context) {
    return Container(
      color: const Color(0xFF0A0E1A),
      child: Column(
        children: [
          _buildHeader(),
          _buildTabs(),
          Expanded(child: _selectedTab == 0 ? _buildList(_installed) : _buildList(_available)),
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
              color: const Color(0xFF8B5CF6).withValues(alpha: 0.15),
            ),
            child: const Icon(Icons.extension_rounded, color: Color(0xFF8B5CF6), size: 24),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Plugins', style: TextStyle(color: Colors.white, fontSize: 20, fontWeight: FontWeight.w700)),
                Text('${_installed.length} installed • ${_available.length} available',
                    style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
              ],
            ),
          ),
          IconButton(
            icon: const Icon(Icons.search_rounded, color: Colors.white54),
            onPressed: () {},
          ),
        ],
      ),
    );
  }

  Widget _buildTabs() {
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 16),
      height: 40,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(10),
        color: const Color(0xFF141829),
      ),
      child: Row(
        children: [
          _tab('Installed', 0),
          _tab('Available', 1),
        ],
      ),
    );
  }

  Widget _tab(String label, int index) {
    final selected = _selectedTab == index;
    return Expanded(
      child: GestureDetector(
        onTap: () => setState(() => _selectedTab = index),
        child: Container(
          alignment: Alignment.center,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(10),
            color: selected ? const Color(0xFF8B5CF6) : Colors.transparent,
          ),
          child: Text(label, style: TextStyle(
            color: selected ? Colors.white : Colors.white.withValues(alpha: 0.5),
            fontSize: 13, fontWeight: FontWeight.w600,
          )),
        ),
      ),
    );
  }

  Widget _buildList(List<_Plugin> plugins) {
    return ListView.builder(
      padding: const EdgeInsets.all(16),
      itemCount: plugins.length,
      itemBuilder: (context, i) {
        final p = plugins[i];
        return Container(
          margin: const EdgeInsets.only(bottom: 10),
          padding: const EdgeInsets.all(14),
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(12),
            color: const Color(0xFF141829),
            border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
          ),
          child: Row(
            children: [
              Container(
                width: 44, height: 44,
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(10),
                  gradient: LinearGradient(
                    colors: [const Color(0xFF8B5CF6).withValues(alpha: 0.2), const Color(0xFF6366F1).withValues(alpha: 0.1)],
                  ),
                ),
                child: Icon(p.icon, color: const Color(0xFF8B5CF6), size: 22),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        Text(p.name, style: const TextStyle(color: Colors.white, fontSize: 14, fontWeight: FontWeight.w600)),
                        const SizedBox(width: 8),
                        Text('v${p.version}', style: TextStyle(color: Colors.white.withValues(alpha: 0.3), fontSize: 11)),
                      ],
                    ),
                    Text(p.description, style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 12)),
                    const SizedBox(height: 4),
                    Row(
                      children: [
                        ...List.generate(5, (si) => Icon(
                          si < p.rating.floor() ? Icons.star_rounded : Icons.star_border_rounded,
                          color: const Color(0xFFF59E0B), size: 14,
                        )),
                        const SizedBox(width: 4),
                        Text(p.rating.toStringAsFixed(1), style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 11)),
                      ],
                    ),
                  ],
                ),
              ),
              if (p.enabled)
                Switch(
                  value: p.enabled,
                  activeThumbColor: const Color(0xFF8B5CF6),
                  onChanged: (v) => setState(() => p.enabled = v),
                )
              else
                SizedBox(
                  height: 32,
                  child: FilledButton(
                    onPressed: () => setState(() => p.enabled = true),
                    style: FilledButton.styleFrom(
                      backgroundColor: const Color(0xFF8B5CF6),
                      padding: const EdgeInsets.symmetric(horizontal: 12),
                      shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(8)),
                    ),
                    child: Text(_selectedTab == 0 ? 'Enable' : 'Install', style: const TextStyle(fontSize: 12)),
                  ),
                ),
            ],
          ),
        );
      },
    );
  }
}

class _Plugin {
  final String name;
  final String description;
  final IconData icon;
  final String version;
  bool enabled;
  final double rating;

  _Plugin(this.name, this.description, this.icon, this.version, this.enabled, this.rating);
}
