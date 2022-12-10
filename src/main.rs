mod schema;

#[tokio::main]
async fn main() {
    let schema = schema::create_schema();

    let sdl = schema.sdl();
    std::fs::write("schema.graphql", sdl).unwrap();
}
