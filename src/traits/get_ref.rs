pub trait GetRef<A, B> {
    fn get_ref(input: &A) -> &B;
}
