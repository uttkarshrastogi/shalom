import 'package:shalom_core/shalom_core.dart';

class Point {
  final int x;
  final int y;
  const Point({required this.x, required this.y});
}

class _PointScalarImpl implements CustomScalarImpl<Point> {
  @override
  Point deserialize(raw) {
    final parts = (raw as String)
        .replaceAll(RegExp(r'[POINT()]'), '')
        .trim()
        .split(' ');
    return Point(x: int.parse(parts[0]), y: int.parse(parts[1]));
  }

  @override
  serialize(Point value) => 'POINT (${value.x} ${value.y})';
}

final pointScalarImpl = _PointScalarImpl();
