import 'package:test/test.dart';
import 'lib/__graphql__/get_location.shalom.dart'; // auto-generated file

void main() {
  test('Point scalar is correctly generated and parsed', () {
    final responseJson = {
      "data": {
        "getLocation": {
          "id": "123",
          "coords": "POINT (12, 34)"
        }
      }
    };

    final result = GraphQLResult<GetLocation$Query$Response>.fromJson(
      responseJson,
          (json) => GetLocation$Query$Response.fromJson(json),
    );

    final location = result.data?.getLocation;
    expect(location, isNotNull);
    expect(location!.coords.x, equals(12));
    expect(location.coords.y, equals(34));
  });
}
