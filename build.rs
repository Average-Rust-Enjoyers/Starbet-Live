fn main() {
    cynic_codegen::register_schema("cloudbet")
        .from_sdl_file("/home/ava/Documents/fi/05s/pv281/Starbet-Live/src/schemas/cloudbet.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
