use anyhow::Result;
use proto_builder_trait::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    let builder = tonic_build::configure();
    builder
        .out_dir("src/pb")
        .with_type_attributes(&["MaterializeRequest"], &[r#"#[derive(Hash, Eq)]"#])
        .compile(
            &[
                "../protos/metadata/messages.proto",
                "../protos/metadata/rpc.proto",
            ],
            &["../protos"],
        )
        .unwrap();
    Ok(())
}
