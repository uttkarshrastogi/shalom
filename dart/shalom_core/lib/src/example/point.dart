// lib/src/point.dart

// If your CostumScalarImpl is in another file, import it.
// Adjust path if scalar.dart is elsewhere

import '../scaler.dart';

class Point {
  final int x;
  final int y;

  const Point({required this.x, required this.y});

  @override
  String toString() => 'Point(x: $x, y: $y)';
}

class _PointScalarImpl implements CustomScalarImpl<Point> {
  @override
  Point deserialize(dynamic raw) {
    // Expects format like: "POINT (12, 34)"
    final regex = RegExp(r'POINT\s*\((\d+),\s*(\d+)\)');
    final match = regex.firstMatch(raw.toString());
    if (match == null) throw FormatException("Invalid POINT format: $raw");
    return Point(x: int.parse(match[1]!), y: int.parse(match[2]!));
  }

  @override
  dynamic serialize(Point value) {
    return "POINT (${value.x}, ${value.y})";
  }
}

// This is referenced in shalom.yml
final pointScalarImpl = _PointScalarImpl();
