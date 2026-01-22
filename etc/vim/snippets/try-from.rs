impl TryFrom<T> for Foo {
  type Error = Bar;

  fn try_from(value: T) -> Result<Self, Self::Error> {
    todo!()
  }
}
