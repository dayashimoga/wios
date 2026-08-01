import 'package:flutter/material.dart';

/// AI Engine page — model management, inference status, and chat.
class AiPage extends StatefulWidget {
  const AiPage({super.key});

  @override
  State<AiPage> createState() => _AiPageState();
}

class _AiPageState extends State<AiPage> {
  final _chatController = TextEditingController();
  final List<_ChatMessage> _messages = [
    _ChatMessage('system', 'WIOS AI Engine ready. Local inference active.'),
  ];

  @override
  void dispose() {
    _chatController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: CustomScrollView(
            slivers: [
              SliverToBoxAdapter(
                child: Padding(
                  padding: const EdgeInsets.all(24),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      _buildHeader(context),
                      const SizedBox(height: 24),
                      _buildModelsCard(),
                      const SizedBox(height: 20),
                      _buildInferenceStats(),
                      const SizedBox(height: 20),
                      _buildChatSection(),
                    ],
                  ),
                ),
              ),
            ],
          ),
        ),
        _buildChatInput(),
      ],
    );
  }

  Widget _buildHeader(BuildContext context) {
    return Row(
      children: [
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('AI Engine',
                style: Theme.of(context).textTheme.headlineMedium?.copyWith(
                      fontWeight: FontWeight.w700, color: Colors.white)),
            const SizedBox(height: 4),
            Text('Local inference • ONNX • TFLite • llama.cpp',
                style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 14)),
          ],
        ),
      ],
    );
  }

  Widget _buildModelsCard() {
    final models = [
      ('TinyLlama 1.1B', 'llama.cpp', '637 MB', true),
      ('MobileNet V3', 'TFLite', '14 MB', true),
      ('Network Anomaly', 'ONNX', '8 MB', false),
    ];
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('Models',
            style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        ...models.map((m) => Container(
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
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(10),
                      color: const Color(0xFFEC4899).withValues(alpha: 0.15),
                    ),
                    child: const Icon(Icons.model_training_rounded, color: Color(0xFFEC4899), size: 20),
                  ),
                  const SizedBox(width: 12),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(m.$1, style: const TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 14)),
                        Text('${m.$2} • ${m.$3}',
                            style: TextStyle(color: Colors.white.withValues(alpha: 0.4), fontSize: 12)),
                      ],
                    ),
                  ),
                  Container(
                    padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(6),
                      color: m.$4
                          ? const Color(0xFF00D4AA).withValues(alpha: 0.1)
                          : Colors.white.withValues(alpha: 0.06),
                    ),
                    child: Text(m.$4 ? 'Loaded' : 'Available',
                        style: TextStyle(
                          color: m.$4 ? const Color(0xFF00D4AA) : Colors.white.withValues(alpha: 0.5),
                          fontSize: 11, fontWeight: FontWeight.w600,
                        )),
                  ),
                ],
              ),
            )),
      ],
    );
  }

  Widget _buildInferenceStats() {
    return Row(
      children: [
        _miniStat('Inferences', '1,284', const Color(0xFFEC4899)),
        const SizedBox(width: 12),
        _miniStat('Avg Latency', '45ms', const Color(0xFF6366F1)),
        const SizedBox(width: 12),
        _miniStat('Queue', '0', const Color(0xFF00D4AA)),
      ],
    );
  }

  Widget _miniStat(String label, String value, Color color) {
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
            Text(value, style: TextStyle(color: color, fontSize: 20, fontWeight: FontWeight.w700)),
            const SizedBox(height: 4),
            Text(label, style: TextStyle(color: Colors.white.withValues(alpha: 0.5), fontSize: 11)),
          ],
        ),
      ),
    );
  }

  Widget _buildChatSection() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const Text('AI Chat',
            style: TextStyle(color: Colors.white, fontWeight: FontWeight.w600, fontSize: 16)),
        const SizedBox(height: 12),
        ..._messages.map((msg) => Padding(
              padding: const EdgeInsets.only(bottom: 8),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Icon(
                    msg.role == 'user' ? Icons.person_rounded : Icons.psychology_rounded,
                    color: msg.role == 'user' ? const Color(0xFF6366F1) : const Color(0xFF00D4AA),
                    size: 18,
                  ),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(msg.text,
                        style: TextStyle(color: Colors.white.withValues(alpha: 0.8), fontSize: 13)),
                  ),
                ],
              ),
            )),
      ],
    );
  }

  Widget _buildChatInput() {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: const Color(0xFF0D1121),
        border: Border(top: BorderSide(color: Colors.white.withValues(alpha: 0.06))),
      ),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _chatController,
              style: const TextStyle(color: Colors.white, fontSize: 14),
              decoration: InputDecoration(
                hintText: 'Ask WIOS AI...',
                hintStyle: TextStyle(color: Colors.white.withValues(alpha: 0.3)),
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                  borderSide: BorderSide(color: Colors.white.withValues(alpha: 0.1)),
                ),
                contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                filled: true,
                fillColor: const Color(0xFF141829),
              ),
              onSubmitted: _sendMessage,
            ),
          ),
          const SizedBox(width: 8),
          IconButton(
            onPressed: () => _sendMessage(_chatController.text),
            icon: const Icon(Icons.send_rounded, color: Color(0xFF00D4AA)),
          ),
        ],
      ),
    );
  }

  void _sendMessage(String text) {
    if (text.trim().isEmpty) return;
    setState(() {
      _messages.add(_ChatMessage('user', text.trim()));
      _messages.add(_ChatMessage('ai', 'Processing locally... (AI backend not yet connected)'));
      _chatController.clear();
    });
  }
}

class _ChatMessage {
  final String role, text;
  _ChatMessage(this.role, this.text);
}
