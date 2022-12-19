mod schema;
#[cfg(test)]
mod schema_utils;

#[tokio::main]
async fn main() {
    let schema = schema::hello::create_schema();

    let sdl = schema.sdl();
    std::fs::write("schema.graphql", sdl).unwrap();
}
