// ignore_for_file: constant_identifier_names, non_constant_identifier_names, unused_import

import 'package:shalom_core/shalom_core.dart';
import '../point.dart' as uomtoe;
// ------------ Enum DEFINITIONS -------------

// ------------ END Enum DEFINITIONS -------------
// ------------ Input DEFINITIONS -------------

class PointDataInput {
  final uomtoe.Point coords;

  final String name;

  PointDataInput({required this.coords, required this.name});

  JsonObject toJson() {
    JsonObject data = {};

    data["coords"] = coords;

    data["name"] = name;

    return data;
  }

  PointDataInput updateWith({uomtoe.Point? coords, String? name}) {
    final uomtoe.Point coords$next;

    if (coords != null) {
      coords$next = coords;
    } else {
      coords$next = this.coords;
    }

    final String name$next;

    if (name != null) {
      name$next = name;
    } else {
      name$next = this.name;
    }

    return PointDataInput(coords: coords$next, name: name$next);
  }
}

class PointUpdateCoordsOpt {
  final Option<uomtoe.Point?> coords;

  final String name;

  PointUpdateCoordsOpt({this.coords = const None(), required this.name});

  JsonObject toJson() {
    JsonObject data = {};

    if (coords.isSome()) {
      data["coords"] = coords.some();
    }

    data["name"] = name;

    return data;
  }

  PointUpdateCoordsOpt updateWith({
    Option<Option<uomtoe.Point?>> coords = const None(),

    String? name,
  }) {
    final Option<uomtoe.Point?> coords$next;

    switch (coords) {
      case Some(value: final data):
        coords$next = data;
      case None():
        coords$next = this.coords;
    }

    final String name$next;

    if (name != null) {
      name$next = name;
    } else {
      name$next = this.name;
    }

    return PointUpdateCoordsOpt(coords: coords$next, name: name$next);
  }
}

// ------------ END Input DEFINITIONS -------------
