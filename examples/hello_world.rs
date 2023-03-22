use mgi::prelude::*;

fn check_resources(world: &mut World) -> MgiResult<()> {
    let title = world.get_resource::<GameTitle>().unwrap();
    assert_eq!(title.0, "Hello World");
    dbg!(&title.0);

    let size = world.get_resource::<GameSize>().unwrap();
    assert_eq!(size.width(), 800);
    assert_eq!(size.height(), 600);
    dbg!((size.width(), size.height()));

    Ok(())
}

fn stop(world: &mut World) -> MgiResult<()> {
    world.stop();
    Ok(())
}

fn main() -> MgiResult<()> {
    GameBuilder::new("Hello World", (800, 600))
        .add_system(Setup, check_resources)
        .add_system(Update, stop)
        .run()?;

    Ok(())
}
