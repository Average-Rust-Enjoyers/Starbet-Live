fn main() {
    cynic_codegen::register_schema("cloudbet")
        .from_sdl_file("./src/schemas/cloudbet.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
