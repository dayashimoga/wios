import 'package:flutter/material.dart';

/// WIOS brand color palette.
class WiosColors {
  WiosColors._();

  // Primary palette — deep cyan/teal gradient
  static const Color primary = Color(0xFF00BCD4);
  static const Color primaryDark = Color(0xFF0097A7);
  static const Color primaryLight = Color(0xFF4DD0E1);

  // Accent — electric blue
  static const Color accent = Color(0xFF448AFF);
  static const Color accentLight = Color(0xFF82B1FF);

  // Surface colors (dark mode)
  static const Color surface = Color(0xFF1E1E2E);
  static const Color surfaceVariant = Color(0xFF2A2A3E);
  static const Color surfaceElevated = Color(0xFF313145);

  // Background
  static const Color background = Color(0xFF11111B);
  static const Color backgroundGradientStart = Color(0xFF0D1117);
  static const Color backgroundGradientEnd = Color(0xFF161B22);

  // Status colors
  static const Color success = Color(0xFF4CAF50);
  static const Color warning = Color(0xFFFF9800);
  static const Color error = Color(0xFFEF5350);
  static const Color info = Color(0xFF42A5F5);

  // Text colors
  static const Color textPrimary = Color(0xFFE1E1E6);
  static const Color textSecondary = Color(0xFF9CA3AF);
  static const Color textMuted = Color(0xFF6B7280);

  // Mesh/network visualization
  static const Color meshNode = Color(0xFF00E5FF);
  static const Color meshEdge = Color(0xFF00BCD4);
  static const Color meshPulse = Color(0xFF18FFFF);

  /// Background gradient for the main app.
  static const LinearGradient backgroundGradient = LinearGradient(
    begin: Alignment.topLeft,
    end: Alignment.bottomRight,
    colors: [backgroundGradientStart, backgroundGradientEnd],
  );

  /// Glassmorphism gradient for cards.
  static LinearGradient get glassGradient => LinearGradient(
        begin: Alignment.topLeft,
        end: Alignment.bottomRight,
        colors: [
          Colors.white.withValues(alpha: 0.08),
          Colors.white.withValues(alpha: 0.03),
        ],
      );
}
