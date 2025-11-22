use serde_json::Value;

pub trait ResourceImpl {
    pub fn build_r4_resource(data: Value);
    pub fn build_r4b_resource(data: Value);
    pub fn build_r4b_resource(data: Value);
    pub fn build_r5_resource(data: Value);
    pub fn build_r6_resource(data: Value);
}
