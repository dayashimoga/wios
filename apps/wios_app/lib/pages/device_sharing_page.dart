import 'package:flutter/material.dart';

/// Device sharing page — share cameras, displays, GPUs, sensors across mesh.
class DeviceSharingPage extends StatefulWidget {
  const DeviceSharingPage({super.key});

  @override
  State<DeviceSharingPage> createState() => _DeviceSharingPageState();
}

class _DeviceSharingPageState extends State<DeviceSharingPage> {
  final List<_SharedDevice> _devices = [
    _SharedDevice('Camera', Icons.videocam_rounded, 'MacBook Pro', true, 1, 3, 'ReadOnly'),
    _SharedDevice('Display', Icons.monitor_rounded, 'Desktop PC', true, 0, 1, 'ReadWrite'),
    _SharedDevice('GPU', Icons.memory_rounded, 'Workstation', false, 0, 2, 'FullControl'),
    _SharedDevice('Microphone', Icons.mic_rounded, 'iPhone 16', true, 2, 5, 'ReadOnly'),
    _SharedDevice('Storage', Icons.storage_rounded, 'NAS Server', true, 3, 10, 'ReadWrite'),
    _SharedDevice('Printer', Icons.print_rounded, 'Office HP', false, 0, 5, 'ReadOnly'),
  ];

  final String _clipboardContent = 'Hello from another device!';

  @override
  Widget build(BuildContext context) {
    return Container(
      color: const Color(0xFF0A0E1A),
      child: Column(
        children: [
          _buildHeader(),
          _buildClipboard(),
          const Padding(
            padding: EdgeInsets.fromLTRB(16, 16, 16, 8),
            child: Align(
              alignment: Alignment.centerLeft,
              child: Text('Shared Resources', style: TextStyle(color: Colors.white, fontSize: 16, fontWeight: FontWeight.w600)),
            ),
          ),
          Expanded(child: _buildDeviceList()),
        ],
      ),
    );
  }

  Widget _buildHeader() {
    final active = _devices.where((d) => d.shared).length;
    return Container(
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          Container(
            padding: const EdgeInsets.all(10),
            decoration: BoxDecoration(
              borderRadius: BorderRadius.circular(12),
              color: const Color(0xFF00D4AA).withValues(alpha: 0.15),
            ),
            child: const Icon(Icons.devices_rounded, color: Color(0xFF00D4AA), size: 24),
          ),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Device Sharing', style: TextStyle(color: Colors.white, fontSize: 20, fontWeight: FontWeight.w700)),
                Text('$active of ${_devices.length} resources active', style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
              ],
            ),
          ),
          FilledButton.icon(
            onPressed: () {},
            icon: const Icon(Icons.add_rounded, size: 18),
            label: const Text('Share'),
            style: FilledButton.styleFrom(
              backgroundColor: const Color(0xFF00D4AA),
              foregroundColor: Colors.black,
              shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(10)),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildClipboard() {
    return Container(
      margin: const EdgeInsets.symmetric(horizontal: 16),
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        color: const Color(0xFF6366F1).withValues(alpha: 0.1),
        border: Border.all(color: const Color(0xFF6366F1).withValues(alpha: 0.2)),
      ),
      child: Row(
        children: [
          const Icon(Icons.content_paste_rounded, color: Color(0xFF6366F1), size: 20),
          const SizedBox(width: 10),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Text('Shared Clipboard', style: TextStyle(color: Color(0xFF6366F1), fontSize: 12, fontWeight: FontWeight.w600)),
                Text(_clipboardContent, style: const TextStyle(color: Colors.white, fontSize: 13), maxLines: 1, overflow: TextOverflow.ellipsis),
              ],
            ),
          ),
          IconButton(
            icon: const Icon(Icons.copy_rounded, color: Color(0xFF6366F1), size: 18),
            onPressed: () {},
          ),
        ],
      ),
    );
  }

  Widget _buildDeviceList() {
    return ListView.builder(
      padding: const EdgeInsets.symmetric(horizontal: 16),
      itemCount: _devices.length,
      itemBuilder: (context, i) {
        final d = _devices[i];
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
                width: 40, height: 40,
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(10),
                  color: (d.shared ? const Color(0xFF00D4AA) : Colors.white.withValues(alpha: 0.1)).withValues(alpha: d.shared ? 0.15 : 1),
                ),
                child: Icon(d.icon, color: d.shared ? const Color(0xFF00D4AA) : Colors.white.withValues(alpha: 0.3), size: 20),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Text(d.name, style: const TextStyle(color: Colors.white, fontSize: 14, fontWeight: FontWeight.w600)),
                    Text('${d.owner} • ${d.permission}', style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 12)),
                    if (d.shared) Text('${d.users}/${d.maxUsers} users', style: TextStyle(color: Colors.white.withValues(alpha: 0.3), fontSize: 11)),
                  ],
                ),
              ),
              Switch(
                value: d.shared,
                activeThumbColor: const Color(0xFF00D4AA),
                onChanged: (v) => setState(() => d.shared = v),
              ),
            ],
          ),
        );
      },
    );
  }
}

class _SharedDevice {
  final String name;
  final IconData icon;
  final String owner;
  bool shared;
  final int users;
  final int maxUsers;
  final String permission;

  _SharedDevice(this.name, this.icon, this.owner, this.shared, this.users, this.maxUsers, this.permission);
}
