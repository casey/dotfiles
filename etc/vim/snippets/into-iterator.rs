impl IntoIterator for Foo {
  type IntoIter = Baz;
  type Item = Bar;

  fn into_iter(self) -> Self::IntoIter {
    todo!()
  }
}
