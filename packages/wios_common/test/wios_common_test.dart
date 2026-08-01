import 'package:flutter_test/flutter_test.dart';
import 'package:wios_common/wios_common.dart';

void main() {
  test('WiosConstants values are correct', () {
    expect(WiosConstants.appName, 'WIOS');
    expect(WiosConstants.version, '0.5.0');
  });
}
