use crate::prelude::*;

pub async fn handle_ia_response(
    ctx: &SerenityContext,
    _framework: FrameworkContext<'_, Data, Error>,
    interaction: &Interaction,
) -> Result<()> {
    let data = interaction.clone().command().unwrap().data;
    if data.name.eq("ask_anything") {
        let mutex = _framework.user_data.ia_response.lock().unwrap().to_string();
        let string = format!("**{mutex}**");
        let embed = CreateEmbed::new().description(string);
        let response = EditInteractionResponse::new().embed(embed);
        let _ = interaction
            .clone()
            .command()
            .unwrap()
            .edit_response(ctx, response)
            .await;
    }
    Ok(())
}