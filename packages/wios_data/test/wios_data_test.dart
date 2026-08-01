import 'package:flutter_test/flutter_test.dart';
import 'package:wios_data/wios_data.dart';

void main() {
  test('MockNodeRepository implements NodeRepository', () async {
    final repo = MockNodeRepository();
    final nodes = await repo.getDiscoveredNodes();
    expect(nodes.isNotEmpty, true);
  });
}
