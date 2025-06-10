import 'package:shalom_core/shalom_core.dart';
import 'package:test/test.dart';
import '__graphql__/GetLocation.shalom.dart';
import 'support/point.dart';

void main() {
  test('Point scalar is correctly generated and parsed', () {
    // Register the scalar handler
    customScalars['Point'] = pointScalarImpl;

    final responseJson = {
      "data": {
        "getLocation": {"id": "123", "coords": "POINT (12, 34)"},
      },
    };

    final result = GraphQLResult<GetLocationResponse>.fromJson(
      responseJson,
      (json) => GetLocationResponse.fromJson(json),
    );

    final location = result.data?.getLocation;

    expect(location, isNotNull);
    expect(location!.id, equals("123"));
    expect(location.coords!.x, equals(12));
    expect(location.coords!.y, equals(34));
  });
}
