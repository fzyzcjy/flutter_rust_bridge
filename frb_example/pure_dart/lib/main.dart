void main() {
  test('dart call simpleAdder', () async {
    expect(await api.simpleAdder(a: 42, b: 100), 142);
  });

  // TODO
}
