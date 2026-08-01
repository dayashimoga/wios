import 'package:flutter/material.dart';

/// Compute page — distributed task monitoring and resource dashboard.
class ComputePage extends StatelessWidget {
  const ComputePage({super.key});

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
                _buildResourceOverview(),
                const SizedBox(height: 20),
                _buildTaskList(),
                const SizedBox(height: 20),
                _buildNodeCapabilities(),
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
            Text('Distributed Compute',
                style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                      fontWeight: FontWeight.w700, color: Colors.white)),
            const SizedBox(height: 4),
            Text('Task scheduling across mesh nodes',
                style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 14)),
          ],
        ),
        const Spacer(),
        FilledButton.icon(
          onPressed: () {},
          icon: const Icon(Icons.add_rounded, size: 18),
          label: const Text('New Task'),
          style: FilledButton.styleFrom(
            backgroundColor: const Color(0xFF8B5CF6),
            foregroundColor: Colors.white,
          ),
        ),
      ],
    );
  }

  Widget _buildResourceOverview() {
    return Row(
      children: [
        _resourceCard('CPU', 0.45, '4 cores', const Color(0xFF8B5CF6)),
        const SizedBox(width: 12),
        _resourceCard('RAM', 0.62, '8 GB', const Color(0xFF00D4AA)),
        const SizedBox(width: 12),
        _resourceCard('GPU', 0.0, 'N/A', const Color(0xFFF59E0B)),
      ],
    );
  }

  Widget _resourceCard(String label, double usage, String info, Color color) {
    return Expanded(
      child: Container(
        padding: const EdgeInsets.all(20),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(12),
          color: const Color(0xFF141829),
          border: Border.all(color: color.withValues(alpha: 0.2)),
        ),
        child: Column(
          children: [
            SizedBox(
              width: 56, height: 56,
              child: Stack(
                children: [
                  CircularProgressIndicator(
                    value: usage,
                    strokeWidth: 5,
                    backgroundColor: Colors.white.withValues(alpha: 0.06),
                    valueColor: AlwaysStoppedAnimation(color),
                  ),
                  Center(
                    child: Text('${(usage * 100).toInt()}%',
                        style: TextStyle(color: color, fontSize: 13, fontWeight: FontWeight.w700)),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 12),
            Text(label, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 14)),
            Text(info, style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 11)),
          ],
        ),
      ),
    );
  }

  Widget _buildTaskList() {
    final tasks = [
      ('AI Inference Batch', 'Running', 0.72, const Color(0xFF00D4AA)),
      ('Data Sync', 'Queued', 0.0, const Color(0xFFF59E0B)),
      ('Model Training', 'Completed', 1.0, const Color(0xFF6366F1)),
    ];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Tasks', style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        ...tasks.map((t) => Container(
              margin: const EdgeInsets.only(bottom: 8),
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                borderRadius: BorderRadius.circular(12),
                color: const Color(0xFF141829),
                border: Border.all(color: Colors.white.withValues(alpha: 0.06)),
              ),
              child: Column(
                children: [
                  Row(
                    children: [
                      Icon(Icons.task_alt_rounded, color: t.$4, size: 20),
                      const SizedBox(width: 12),
                      Expanded(child: Text(t.$1, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600))),
                      Container(
                        padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 3),
                        decoration: BoxDecoration(
                          borderRadius: BorderRadius.circular(6),
                          color: t.$4.withValues(alpha: 0.1),
                        ),
                        child: Text(t.$2, style: TextStyle(color: t.$4, fontSize: 11, fontWeight: FontWeight.w600)),
                      ),
                    ],
                  ),
                  if (t.$3 > 0 && t.$3 < 1) ...[
                    const SizedBox(height: 10),
                    ClipRRect(
                      borderRadius: BorderRadius.circular(4),
                      child: LinearProgressIndicator(
                        value: t.$3,
                        minHeight: 4,
                        backgroundColor: Colors.white.withValues(alpha: 0.06),
                        valueColor: AlwaysStoppedAnimation(t.$4),
                      ),
                    ),
                  ],
                ],
              ),
            )),
      ],
    );
  }

  Widget _buildNodeCapabilities() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Mesh Nodes', style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        _nodeTile('Local Node', '4 CPU • 8 GB RAM', true),
        _nodeTile('Kitchen Hub', '2 CPU • 4 GB RAM', true),
        _nodeTile('Office PC', '8 CPU • 16 GB RAM • GPU', false),
      ],
    );
  }

  Widget _nodeTile(String name, String caps, bool available) {
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
          Icon(Icons.computer_rounded, color: available ? const Color(0xFF00D4AA) : Colors.white.withValues(alpha: 0.3), size: 20),
          const SizedBox(width: 12),
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(name, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 13)),
                Text(caps, style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 11)),
              ],
            ),
          ),
          Container(
            width: 8, height: 8,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              color: available ? const Color(0xFF00D4AA) : Colors.white.withValues(alpha: 0.2),
            ),
          ),
        ],
      ),
    );
  }
}
