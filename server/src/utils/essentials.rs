
/// A type that can be either `S` or `T`.
pub enum TypeOr<S, T> {
    Left(S),
    Right(T),
}
