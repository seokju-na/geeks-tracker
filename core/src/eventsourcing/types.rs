use typeshare::typeshare;

#[typeshare(serialized_as = "i32")]
pub type Version = u64;
#[typeshare(serialized_as = "i32")]
pub type Timestamp = i64;
