import 'package:flutter_test/flutter_test.dart';
import 'package:wios_app/main.dart';

void main() {
  testWidgets('WIOS App builds successfully', (WidgetTester tester) async {
    await tester.pumpWidget(const WiosApp());
    expect(find.byType(WiosApp), findsOneWidget);
  });
}
