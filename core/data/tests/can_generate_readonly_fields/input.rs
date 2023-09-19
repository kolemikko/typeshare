#[typeshare]
pub struct SomeStruct {
    #[typeshare(typescript(readonly), csharp(readonly))]
    field_a: u32,
}
