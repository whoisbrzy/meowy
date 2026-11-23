require('dotenv').config();
const { Client, GatewayIntentBits, Collection } = require('discord.js');
const { commands } = require('./commands');

if (!process.env.DISCORD_TOKEN) throw new Error('Missing DISCORD_TOKEN in .env');
const client = new Client({ intents: [GatewayIntentBits.Guilds] });
client.commands = new Collection(commands.map(c => [c.name, c]));

client.once('ready', () => console.log(`Meowy online as ${client.user.tag}`));
client.on('interactionCreate', async interaction => {
  if (!interaction.isChatInputCommand()) return;
  const command = client.commands.get(interaction.commandName);
  if (!command) return;
  try { await command.execute(interaction); }
  catch (err) {
    console.error(err);
    const msg = 'Something went wrong while running that command.';
    if (interaction.replied || interaction.deferred) await interaction.followUp({ content: msg, ephemeral: true });
    else await interaction.reply({ content: msg, ephemeral: true });
  }
});
client.login(process.env.DISCORD_TOKEN);
