import 'package:flutter/material.dart';
import '../theme/wios_colors.dart';

/// Animated status indicator dot.
class StatusIndicator extends StatelessWidget {
  final bool isOnline;
  final double size;

  const StatusIndicator({
    super.key,
    required this.isOnline,
    this.size = 10,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      width: size,
      height: size,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        color: isOnline ? WiosColors.success : WiosColors.error,
        boxShadow: isOnline
            ? [
                BoxShadow(
                  color: WiosColors.success.withValues(alpha: 0.5),
                  blurRadius: size,
                  spreadRadius: size * 0.2,
                ),
              ]
            : null,
      ),
    );
  }
}
