int findMatchingBracket(String text, int start) {
  var depth = 0;
  if (text[start] != '{') throw Exception();
  for (var i = start; i < text.length; ++i) {
    switch (text[i]) {
      case '{':
        depth++;
      case '}':
        depth--;
        if (depth == 0) return i;
    }
  }
  throw Exception('not found');
}
