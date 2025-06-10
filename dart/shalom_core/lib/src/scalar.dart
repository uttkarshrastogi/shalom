abstract class CustomScalarImpl<T> {
  T deserialize(dynamic raw);
  dynamic serialize(T value);
}

// Global registry to hold custom scalar implementations
final Map<String, CustomScalarImpl<dynamic>> customScalars = {};
