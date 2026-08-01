import 'package:flutter/material.dart';

/// Messages page — encrypted messaging with gossipsub topics.
class MessagesPage extends StatefulWidget {
  const MessagesPage({super.key});

  @override
  State<MessagesPage> createState() => _MessagesPageState();
}

class _MessagesPageState extends State<MessagesPage> {
  final _msgController = TextEditingController();
  int _selectedChannel = 0;

  final _channels = ['General', 'Mesh Alerts', 'AI Logs', 'Direct'];
  final List<_Msg> _messages = [
    _Msg('System', 'Gossipsub broker initialized', '12:00', true),
    _Msg('Kitchen Hub', 'Temperature: 23.4°C, Humidity: 45%', '12:01', false),
    _Msg('Office PC', 'Sync complete — 42 files updated', '12:02', false),
    _Msg('AI Engine', 'Anomaly detected: unusual network traffic from 10.0.0.15', '12:03', true),
  ];

  @override
  void dispose() {
    _msgController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        _buildChannelBar(),
        Expanded(
          child: ListView.builder(
            padding: const EdgeInsets.all(16),
            itemCount: _messages.length,
            itemBuilder: (context, i) => _buildMessageBubble(_messages[i]),
          ),
        ),
        _buildInput(),
      ],
    );
  }

  Widget _buildChannelBar() {
    return Container(
      height: 56,
      padding: const EdgeInsets.symmetric(horizontal: 16),
      decoration: BoxDecoration(
        color: const Color(0xFF0D1121),
        border: Border(bottom: BorderSide(color: Colors.white.withValues(alpha: 0.06))),
      ),
      child: Row(
        children: [
          const Icon(Icons.lock_rounded, color: Color(0xFF00D4AA), size: 16),
          const SizedBox(width: 8),
          const Text('E2E Encrypted',
              style: TextStyle(color: Color(0xFF00D4AA), fontSize: 12, fontWeight: FontWeight.w600)),
          const SizedBox(width: 16),
          Expanded(
            child: ListView.builder(
              scrollDirection: Axis.horizontal,
              itemCount: _channels.length,
              itemBuilder: (context, i) {
                final selected = i == _selectedChannel;
                return GestureDetector(
                  onTap: () => setState(() => _selectedChannel = i),
                  child: Container(
                    margin: const EdgeInsets.symmetric(horizontal: 4, vertical: 10),
                    padding: const EdgeInsets.symmetric(horizontal: 14),
                    decoration: BoxDecoration(
                      borderRadius: BorderRadius.circular(8),
                      color: selected
                          ? const Color(0xFF6366F1).withValues(alpha: 0.2)
                          : Colors.transparent,
                      border: Border.all(
                        color: selected
                            ? const Color(0xFF6366F1).withValues(alpha: 0.4)
                            : Colors.white.withValues(alpha: 0.08),
                      ),
                    ),
                    child: Center(
                      child: Text(_channels[i],
                          style: TextStyle(
                            color: selected ? const Color(0xFF6366F1) : Colors.white.withValues(alpha: 0.5),
                            fontSize: 12, fontWeight: FontWeight.w600,
                          )),
                    ),
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildMessageBubble(_Msg msg) {
    return Container(
      margin: const EdgeInsets.only(bottom: 10),
      padding: const EdgeInsets.all(14),
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(12),
        color: msg.isSystem
            ? const Color(0xFF00D4AA).withValues(alpha: 0.06)
            : const Color(0xFF141829),
        border: Border.all(
          color: msg.isSystem
              ? const Color(0xFF00D4AA).withValues(alpha: 0.15)
              : Colors.white.withValues(alpha: 0.06),
        ),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Text(msg.sender,
                  style: TextStyle(
                    color: msg.isSystem ? const Color(0xFF00D4AA) : const Color(0xFF6366F1),
                    fontWeight: FontWeight.w600, fontSize: 12,
                  )),
              const Spacer(),
              Text(msg.time,
                  style: TextStyle(color: Colors.white.withValues(alpha: 0.3), fontSize: 10)),
            ],
          ),
          const SizedBox(height: 6),
          Text(msg.text, style: TextStyle(color: Colors.white.withValues(alpha: 0.8), fontSize: 13)),
        ],
      ),
    );
  }

  Widget _buildInput() {
    return Container(
      padding: const EdgeInsets.all(12),
      decoration: BoxDecoration(
        color: const Color(0xFF0D1121),
        border: Border(top: BorderSide(color: Colors.white.withValues(alpha: 0.06))),
      ),
      child: Row(
        children: [
          Expanded(
            child: TextField(
              controller: _msgController,
              style: const TextStyle(color: Colors.white, fontSize: 14),
              decoration: InputDecoration(
                hintText: 'Send encrypted message...',
                hintStyle: TextStyle(color: Colors.white.withValues(alpha: 0.3)),
                border: OutlineInputBorder(
                  borderRadius: BorderRadius.circular(12),
                  borderSide: BorderSide.none,
                ),
                filled: true,
                fillColor: const Color(0xFF141829),
                contentPadding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
              ),
              onSubmitted: _send,
            ),
          ),
          const SizedBox(width: 8),
          IconButton(
            onPressed: () => _send(_msgController.text),
            icon: const Icon(Icons.send_rounded, color: Color(0xFF6366F1)),
          ),
        ],
      ),
    );
  }

  void _send(String text) {
    if (text.trim().isEmpty) return;
    setState(() {
      _messages.add(_Msg('You', text.trim(), '${TimeOfDay.now().format(context)}', false));
      _msgController.clear();
    });
  }
}

class _Msg {
  final String sender, text, time;
  final bool isSystem;
  _Msg(this.sender, this.text, this.time, this.isSystem);
}
