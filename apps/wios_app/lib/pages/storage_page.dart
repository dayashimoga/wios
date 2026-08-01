import 'package:flutter/material.dart';

/// Storage page — shows usage, quotas, files, and sync status.
class StoragePage extends StatelessWidget {
  const StoragePage({super.key});

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
                _buildUsageCard(),
                const SizedBox(height: 20),
                _buildQuotaBreakdown(),
                const SizedBox(height: 20),
                _buildRecentFiles(),
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
            Text('Storage',
                style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                      fontWeight: FontWeight.w700, color: Colors.white)),
            const SizedBox(height: 4),
            Text('Encrypted, chunked, deduplicated',
                style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 14)),
          ],
        ),
        const Spacer(),
        FilledButton.icon(
          onPressed: () {},
          icon: const Icon(Icons.sync_rounded, size: 18),
          label: const Text('Sync'),
          style: FilledButton.styleFrom(
            backgroundColor: const Color(0xFFF59E0B),
            foregroundColor: Colors.black,
          ),
        ),
      ],
    );
  }

  Widget _buildUsageCard() {
    const used = 1.2;
    const total = 5.0;
    const ratio = used / total;
    return Container(
      padding: const EdgeInsets.all(24),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(16),
        gradient: LinearGradient(
          colors: [
            const Color(0xFFF59E0B).withValues(alpha: 0.12),
            const Color(0xFF141829),
          ],
        ),
        border: Border.all(color: const Color(0xFFF59E0B).withValues(alpha: 0.2)),
      ),
      child: Column(
        children: [
          Row(
            children: [
              const Icon(Icons.sd_storage_rounded, color: Color(0xFFF59E0B), size: 28),
              const SizedBox(width: 12),
              const Text('${used} GB', style: TextStyle(color: Colors.white, fontSize: 28, fontWeight: FontWeight.w700)),
              Text(' / ${total} GB',
                  style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 16)),
              const Spacer(),
              Container(
                padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 5),
                decoration: BoxDecoration(
                  borderRadius: BorderRadius.circular(8),
                  color: const Color(0xFF00D4AA).withValues(alpha: 0.15),
                ),
                child: const Text('Healthy', style: TextStyle(color: Color(0xFF00D4AA), fontSize: 12, fontWeight: FontWeight.w600)),
              ),
            ],
          ),
          const SizedBox(height: 16),
          ClipRRect(
            borderRadius: BorderRadius.circular(6),
            child: LinearProgressIndicator(
              value: ratio,
              minHeight: 8,
              backgroundColor: Colors.white.withValues(alpha: 0.06),
              valueColor: const AlwaysStoppedAnimation(Color(0xFFF59E0B)),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildQuotaBreakdown() {
    final items = [
      ('Documents', '420 MB', Icons.description_rounded, const Color(0xFF6366F1)),
      ('Media', '310 MB', Icons.image_rounded, const Color(0xFFEC4899)),
      ('Models', '280 MB', Icons.model_training_rounded, const Color(0xFF00D4AA)),
      ('Messages', '190 MB', Icons.chat_rounded, const Color(0xFFF59E0B)),
    ];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Breakdown',
            style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        ...items.map((item) => _buildBreakdownTile(item.$1, item.$2, item.$3, item.$4)),
      ],
    );
  }

  Widget _buildBreakdownTile(String label, String size, IconData icon, Color color) {
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
          Expanded(child: Text(label, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600))),
          Text(size, style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 13)),
        ],
      ),
    );
  }

  Widget _buildRecentFiles() {
    final files = [
      ('meeting_notes.md', '12 KB', 'Just now', Icons.description_rounded),
      ('network_scan.json', '45 KB', '5 min ago', Icons.data_object_rounded),
      ('model_v2.onnx', '124 MB', '1 hour ago', Icons.model_training_rounded),
    ];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Recent Files',
            style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        ...files.map((f) => Container(
              margin: const EdgeInsets.only(bottom: 8),
              padding: const EdgeInsets.all(14),
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(12),
                color: const Color(0xFF141829),
                border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
              ),
              child: Row(
                children: [
                  Icon(f.$4, color: const Color(0xFF6366F1), size: 20),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(f.$1, style: const TextStyle(color: Colors.white, fontSize: 13, fontWeight: FontWeight.w500)),
                        Text(f.$2, style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 11)),
                      ],
                    ),
                  ),
                  Text(f.$3, style: TextStyle(color: Colors.white.withValues(alpha: 0.3), fontSize: 11)),
                  const SizedBox(width: 8),
                  const Icon(Icons.lock_rounded, color: Color(0xFF00D4AA), size: 14),
                ],
              ),
            )),
      ],
    );
  }
}
