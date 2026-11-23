use poise::serenity_prelude as serenity;

/// Render all bot replies through the Components V2 Container/TextDisplay pattern.
pub fn text(text: impl Into<String>) -> poise::CreateReply {
    let children = vec![serenity::CreateContainerComponent::TextDisplay(
        serenity::CreateTextDisplay::new(text.into()),
    )];
    poise::CreateReply::default()
        .flags(serenity::MessageFlags::IS_COMPONENTS_V2)
        .components(vec![serenity::CreateComponent::Container(
            serenity::CreateContainer::new(children),
        )])
}
