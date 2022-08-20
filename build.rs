fn main() -> anyhow::Result<()> {
    let mut config = vergen::Config::default();

    *config.git_mut().sha_kind_mut() = vergen::ShaKind::Short;
    *config.git_mut().semver_dirty_mut() = Some("-dirty");

    vergen::vergen(config)
}