import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';
import 'wios_colors.dart';

/// WIOS Material 3 theme configuration.
class WiosTheme {
  WiosTheme._();

  static ThemeData get dark => ThemeData(
        useMaterial3: true,
        brightness: Brightness.dark,
        colorScheme: const ColorScheme.dark(
          primary: WiosColors.primary,
          secondary: WiosColors.accent,
          surface: WiosColors.surface,
          error: WiosColors.error,
          onPrimary: Colors.black,
          onSecondary: Colors.white,
          onSurface: WiosColors.textPrimary,
          onError: Colors.white,
        ),
        scaffoldBackgroundColor: WiosColors.background,
        textTheme: GoogleFonts.interTextTheme(
          ThemeData.dark().textTheme,
        ).apply(
          bodyColor: WiosColors.textPrimary,
          displayColor: WiosColors.textPrimary,
        ),
        appBarTheme: AppBarTheme(
          backgroundColor: WiosColors.surface.withValues(alpha: 0.8),
          elevation: 0,
          centerTitle: false,
          titleTextStyle: GoogleFonts.inter(
            fontSize: 20,
            fontWeight: FontWeight.w600,
            color: WiosColors.textPrimary,
          ),
        ),
        cardTheme: CardThemeData(
          color: WiosColors.surfaceVariant,
          elevation: 0,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
          ),
        ),
        navigationRailTheme: const NavigationRailThemeData(
          backgroundColor: WiosColors.surface,
          indicatorColor: WiosColors.primary,
          selectedIconTheme: IconThemeData(color: Colors.black),
          unselectedIconTheme: IconThemeData(color: WiosColors.textSecondary),
        ),
        elevatedButtonTheme: ElevatedButtonThemeData(
          style: ElevatedButton.styleFrom(
            backgroundColor: WiosColors.primary,
            foregroundColor: Colors.black,
            shape: RoundedRectangleBorder(
              borderRadius: BorderRadius.circular(12),
            ),
            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 14),
          ),
        ),
        inputDecorationTheme: InputDecorationTheme(
          filled: true,
          fillColor: WiosColors.surfaceElevated,
          border: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: BorderSide.none,
          ),
          focusedBorder: OutlineInputBorder(
            borderRadius: BorderRadius.circular(12),
            borderSide: const BorderSide(color: WiosColors.primary),
          ),
        ),
        dividerTheme: DividerThemeData(
          color: WiosColors.textMuted.withValues(alpha: 0.2),
          thickness: 1,
        ),
      );
}
