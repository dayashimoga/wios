import 'package:flutter_test/flutter_test.dart';
import 'package:wios_common/wios_common.dart';

void main() {
  test('WiosConstants values are correct', () {
    expect(WiosConstants.appName, 'WIOS');
    expect(WiosConstants.appVersion, '0.1.0');
  });
}
