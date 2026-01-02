fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(
            &[
                "proto/mdg.proto",
                "proto/events.proto",
            ],
            &["proto"],
        )?;
    Ok(())
}
