import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:wios_app/pages/mesh_page.dart';
import 'package:wios_app/pages/storage_page.dart';
import 'package:wios_app/pages/settings_page.dart';
import 'package:wios_app/pages/compute_page.dart';

void main() {
  group('MeshPage', () {
    testWidgets('renders header and scan button', (tester) async {
      await tester.pumpWidget(const MaterialApp(home: Scaffold(body: MeshPage())));
      await tester.pump(const Duration(milliseconds: 500));
      expect(find.text('Mesh Network'), findsOneWidget);
      expect(find.text('Scan'), findsOneWidget);
    });

    testWidgets('scan button toggles to stop', (tester) async {
      await tester.pumpWidget(const MaterialApp(home: Scaffold(body: MeshPage())));
      await tester.pump(const Duration(milliseconds: 500));
      await tester.tap(find.text('Scan'));
      await tester.pump(const Duration(milliseconds: 500));
      expect(find.text('Stop'), findsOneWidget);
    });
  });

  group('StoragePage', () {
    testWidgets('renders storage header and sync button', (tester) async {
      await tester.pumpWidget(const MaterialApp(home: Scaffold(body: StoragePage())));
      await tester.pumpAndSettle();
      expect(find.text('Storage'), findsOneWidget);
      expect(find.text('Sync'), findsOneWidget);
    });

    testWidgets('shows breakdown section', (tester) async {
      await tester.pumpWidget(const MaterialApp(home: Scaffold(body: StoragePage())));
      await tester.pumpAndSettle();
      expect(find.text('Breakdown'), findsOneWidget);
      expect(find.text('Documents'), findsOneWidget);
    });
  });

  group('ComputePage', () {
    testWidgets('renders compute dashboard', (tester) async {
      await tester.pumpWidget(const MaterialApp(home: Scaffold(body: ComputePage())));
      await tester.pumpAndSettle();
      expect(find.text('Distributed Compute'), findsOneWidget);
      expect(find.text('New Task'), findsOneWidget);
      expect(find.text('Tasks'), findsOneWidget);
    });
  });

  group('SettingsPage', () {
    testWidgets('renders settings with toggles', (tester) async {
      await tester.pumpWidget(const MaterialApp(home: Scaffold(body: SettingsPage())));
      await tester.pumpAndSettle();
      expect(find.text('Settings'), findsOneWidget);
      expect(find.text('Security'), findsOneWidget);
      expect(find.text('End-to-End Encryption'), findsOneWidget);
    });

    testWidgets('toggle switches work', (tester) async {
      await tester.pumpWidget(const MaterialApp(home: Scaffold(body: SettingsPage())));
      await tester.pumpAndSettle();
      final switches = find.byType(Switch);
      expect(switches, findsWidgets);
    });
  });
}
