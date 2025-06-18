import 'package:test/test.dart';
import 'package:shalom_core/shalom_core.dart'; // Assuming Shalom's core types are here

// Import your custom scalar Dart class
import 'point.dart'; // Adjust path if your point.dart is elsewhere

// Import the generated GraphQL operation files, adhering to your naming from CustomScalarInputs.graphql

void main() {
  // Define a sample Point object and its raw string representation for testing
  final Point samplePoint = Point(x: 10, y: 20);
  final String samplePointRaw = "POINT (10, 20)";

  final Point updatedPoint = Point(x: 30, y: 40);
  final String updatedPointRaw = "POINT (30, 40)";

  test("required custom scalar argument", () {
    // Matches "required enum argument"
    var variables = UpdatePointCoordsNonNullVariables(coords: samplePoint);
    var variablesUpdated = variables.updateWith(coords: updatedPoint);
    expect(variablesUpdated.coords, updatedPoint);
    expect(variables.toJson(), {
      "coords": samplePointRaw,
    }); // Matches toJson usage
    final req =
        RequestUpdatePointCoordsNonNull(
          variables: UpdatePointCoordsNonNullVariables(coords: updatedPoint),
        ).toRequest();
    expect(req.variables, {"coords": updatedPointRaw});
  });

  test("optional custom scalar argument", () {
    // Matches "optional enum argument"
    var variables = UpdatePointCoordsOptVariables(
      coords: Some(samplePoint),
    ); // Using Some()
    var variablesUpdated = variables.updateWith(
      coords: Some(Some(updatedPoint)), // Nested Some() for optional update
    );
    expect(
      variablesUpdated.coords?.some(),
      updatedPoint,
    ); // Accessing value with .some()

    // Test explicit null
    expect(
      RequestUpdatePointCoordsOpt(
        variables: UpdatePointCoordsOptVariables(
          coords: Some(null),
        ), // Explicit null
      ).toRequest().variables,
      {"coords": null},
    );

    // Test with value
    expect(
      RequestUpdatePointCoordsOpt(
        variables: UpdatePointCoordsOptVariables(coords: Some(samplePoint)),
      ).toRequest().variables,
      {"coords": samplePointRaw},
    );

    // Test omitted (None())
    expect(
      RequestUpdatePointCoordsOpt(
        variables: UpdatePointCoordsOptVariables(
          coords: None(),
        ), // No value (omitted)
      ).toRequest().variables,
      {},
    );
  });

  test("required custom scalar argument in InputObject", () {
    // Matches "required enum argument in InputObject"
    final variables = UpdatePointWithInputNonNullVariables(
      pointData: PointDataInput(coords: samplePoint, name: "Location A"),
    );
    final updatedPointData = PointDataInput(
      coords: updatedPoint,
      name: "Location B",
    );
    final variablesUpdated = variables.updateWith(pointData: updatedPointData);
    expect(variablesUpdated.pointData.coords, updatedPoint);

    final req =
        RequestUpdatePointWithInputNonNull(
          variables: UpdatePointWithInputNonNullVariables(
            pointData: PointDataInput(
              coords: Point(x: 50, y: 60),
              name: "Location C",
            ),
          ),
        ).toRequest();
    expect(req.variables, {
      "pointData": {"coords": "POINT (50, 60)", "name": "Location C"},
    });
  });

  test("optional custom scalar argument in InputObject", () {
    // Matches "optional enum argument in InputObject"
    // Test with explicit null
    final variables = UpdatePointWithInputCoordsOptVariables(
      pointData: PointUpdateCoordsOpt(coords: Some(null), name: "Location D"),
    );
    final variablesUpdated = variables.updateWith(
      pointData: PointUpdateCoordsOpt(
        coords: Some(updatedPoint),
        name: "Location E",
      ),
    );
    expect(
      variablesUpdated.pointData.coords,
      Some<Point?>(updatedPoint),
    ); // Asserting with Some<T?>

    final req =
        RequestUpdatePointWithInputCoordsOpt(variables: variables).toRequest();
    expect(req.variables, {
      "pointData": {"coords": null, "name": "Location D"},
    });

    // Test with a value
    final variablesWithValue = UpdatePointWithInputCoordsOptVariables(
      pointData: PointUpdateCoordsOpt(
        coords: Some(samplePoint),
        name: "Location F",
      ),
    );
    final reqWithValue =
        RequestUpdatePointWithInputCoordsOpt(
          variables: variablesWithValue,
        ).toRequest();
    expect(reqWithValue.variables, {
      "pointData": {"coords": samplePointRaw, "name": "Location F"},
    });

    // Test omitted (None()) inside input object
    final variablesOmitted = UpdatePointWithInputCoordsOptVariables(
      pointData: PointUpdateCoordsOpt(coords: None(), name: "Location G"),
    );
    final reqOmitted =
        RequestUpdatePointWithInputCoordsOpt(
          variables: variablesOmitted,
        ).toRequest();
    expect(reqOmitted.variables, {
      "pointData": {"name": "Location G"}, // 'coords' should be omitted
    });
  });

  test("optional custom scalar argument with default value", () {
    // Matches "optional enum argument with default value"
    // For a query with a default value, if the variable is not provided by the client,
    // the server applies the default. The client should send an empty map for that variable.
    final req =
        GetPointWithDefaultCoordsRequest(
          variables: GetPointWithDefaultCoordsVariables(id: "test-id-1"),
        ).toRequest();
    expect(req.variables, {
      "id": "test-id-1",
    }); // No 'coords' should be sent if relying on default

    // If you explicitly provide the default value, it should be serialized
    final reqWithExplicitDefault =
        GetPointWithDefaultCoordsRequest(
          variables: GetPointWithDefaultCoordsVariables(
            id: "test-id-2",
            coords: Some(Point(x: 0, y: 0)),
          ),
        ).toRequest();
    expect(reqWithExplicitDefault.variables, {
      "id": "test-id-2",
      "coords": "POINT (0,0)",
    });

    // If you provide a custom value, it should be serialized
    final reqWithCustomValue =
        GetPointWithDefaultCoordsRequest(
          variables: GetPointWithDefaultCoordsVariables(
            id: "test-id-3",
            coords: Some(Point(x: 50, y: 60)),
          ),
        ).toRequest();
    expect(reqWithCustomValue.variables, {
      "id": "test-id-3",
      "coords": "POINT (50, 60)",
    });
  });
}
